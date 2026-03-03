use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// A boxed factory that produces a type-erased Arc instance.
type Factory = Arc<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>;

/// A compile-time safe, type-keyed dependency injection container.
///
/// Supports two service lifetimes:
/// - **Singleton**: one shared instance for the entire application.
/// - **Scoped**: one fresh instance per [`ScopedContainer`] (typically per request).
///
/// No global state — the container is always passed explicitly via [`Arc`].
#[derive(Default)]
pub struct Container {
    singletons: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    scoped_factories: HashMap<TypeId, Factory>,
}

impl Container {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a singleton service (shared across all requests and threads).
    pub fn register<T: Send + Sync + 'static>(&mut self, service: T) {
        self.singletons.insert(TypeId::of::<T>(), Arc::new(service));
    }

    /// Register a singleton service already wrapped in [`Arc`].
    pub fn register_arc<T: Send + Sync + 'static>(&mut self, service: Arc<T>) {
        self.singletons.insert(TypeId::of::<T>(), service);
    }

    /// Register a scoped service with a factory.
    ///
    /// A new instance is created once per [`ScopedContainer`] on first resolution,
    /// then cached for the lifetime of that scope.
    pub fn register_scoped<T: Send + Sync + 'static>(
        &mut self,
        factory: impl Fn() -> T + Send + Sync + 'static,
    ) {
        let factory: Factory = Arc::new(move || Arc::new(factory()));
        self.scoped_factories.insert(TypeId::of::<T>(), factory);
    }

    /// Resolve a singleton service. Returns [`None`] if not registered.
    pub fn resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.singletons
            .get(&TypeId::of::<T>())
            .and_then(|s| s.clone().downcast::<T>().ok())
    }

    // Used by ScopedContainer — not part of the public API surface for consumers.
    pub(crate) fn scoped_factory(&self, type_id: TypeId) -> Option<Factory> {
        self.scoped_factories.get(&type_id).cloned()
    }
}

/// A per-request (or per-scope) dependency container.
///
/// Scoped services are instantiated once on first resolution and cached
/// for the lifetime of this scope. Singleton resolution always delegates
/// to the parent [`Container`].
///
/// # Thread Safety
///
/// Internally uses a [`RwLock`] with double-checked locking so the same
/// `Arc<ScopedContainer>` may be shared across tasks spawned within one request.
pub struct ScopedContainer {
    parent: Arc<Container>,
    cache: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ScopedContainer {
    pub fn new(parent: Arc<Container>) -> Self {
        Self {
            parent,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Resolve a service from this scope.
    ///
    /// Resolution order:
    /// 1. Scoped cache (already instantiated this scope).
    /// 2. Scoped factory in parent → instantiate & cache.
    /// 3. Singleton in parent container.
    pub fn resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();

        // Fast path: read lock — most calls hit this.
        if let Some(svc) = self.cache.read().unwrap().get(&type_id) {
            return svc.clone().downcast::<T>().ok();
        }

        // Clone the factory *before* taking the write lock to minimise contention.
        if let Some(factory) = self.parent.scoped_factory(type_id) {
            let instance = factory();
            let mut cache = self.cache.write().unwrap();
            // Double-check: another task may have populated the cache while we
            // were constructing the instance.
            if let Some(svc) = cache.get(&type_id) {
                return svc.clone().downcast::<T>().ok();
            }
            let result = instance.clone().downcast::<T>().ok();
            cache.insert(type_id, instance);
            return result;
        }

        // Fall back to singleton.
        self.parent.resolve::<T>()
    }

    /// Resolve a singleton directly from the root container, bypassing the
    /// scoped cache. Equivalent to `parent.resolve::<T>()`.
    pub fn resolve_singleton<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.parent.resolve::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyService(u32);

    #[test]
    fn test_singleton_registration() {
        let mut container = Container::new();
        container.register(MyService(10));

        let svc = container
            .resolve::<MyService>()
            .expect("Should resolve singleton");
        assert_eq!(svc.0, 10);
    }

    #[test]
    fn test_scoped_registration() {
        let mut container = Container::new();
        container.register_scoped(|| MyService(20));
        let container = Arc::new(container);

        let scope1 = ScopedContainer::new(Arc::clone(&container));
        let svc1 = scope1
            .resolve::<MyService>()
            .expect("Should resolve in scope 1");
        assert_eq!(svc1.0, 20);

        let scope2 = ScopedContainer::new(Arc::clone(&container));
        let svc2 = scope2
            .resolve::<MyService>()
            .expect("Should resolve in scope 2");
        assert_eq!(svc2.0, 20);

        // Ensure they are distinct instances if not cached correctly (factory creates new)
        // Note: the factory closure captures 20, but creates a NEW instance each time it's called.
        // The ScopedContainer CACHES it.
        assert!(
            !Arc::ptr_eq(&svc1, &svc2),
            "Scoped instances should be different across scopes"
        );

        let svc1_again = scope1
            .resolve::<MyService>()
            .expect("Should resolve again in scope 1");
        assert!(
            Arc::ptr_eq(&svc1, &svc1_again),
            "Should be cached within the same scope"
        );
    }
}
