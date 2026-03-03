extern crate proc_macro;

mod controller;
mod model;
mod route;

use proc_macro::TokenStream;

// ── Standalone route attributes ───────────────────────────────────────────────
//
// When applied to a **free async function**, each attribute generates a
// companion `{FnName}Route` struct that implements `craken_http::RouteProvider`,
// making the function mountable via `HttpServer::configure_routes`.
//
// When the same attribute appears on a **method inside a `#[controller]` impl
// block**, it is treated as a pure marker: `#[controller]` collects and strips
// these attributes to build the `RouteProvider` impl, so they are never
// independently invoked.

/// Register a `GET` route.
///
/// # Standalone usage
/// ```rust,ignore
/// #[get("/health")]
/// pub async fn health() -> Json<serde_json::Value> { … }
///
/// // Generates `HealthRoute` — use with `HttpServer::configure_routes(&HealthRoute)`.
/// ```
///
/// # Inside `#[controller]`
/// ```rust,ignore
/// #[controller]
/// impl MyController {
///     #[get("/items")]
///     pub async fn index(Inject(svc): Inject<ItemService>) -> Json<Vec<Item>> { … }
/// }
/// ```
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand("get", args, input)
}

/// Register a `POST` route.
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand("post", args, input)
}

/// Register a `PUT` route.
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand("put", args, input)
}

/// Register a `DELETE` route.
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand("delete", args, input)
}

/// Register a `PATCH` route.
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand("patch", args, input)
}

// ── Controller macro ──────────────────────────────────────────────────────────

/// Derive a [`craken_http::RouteProvider`] implementation for a controller
/// impl block.
///
/// Scans every method annotated with `#[get]`, `#[post]`, `#[put]`,
/// `#[delete]`, or `#[patch]` and generates a `RouteProvider::routes()`
/// implementation that mounts each method at its declared path.
///
/// Route-attribute annotations are **stripped** from the final code so the
/// compiler never sees them as unknown attributes.
///
/// Any `self` / `&self` / `&mut self` receiver on annotated methods is also
/// stripped: controller methods must obtain all dependencies via axum
/// extractors (`Inject<T>`, `RequestContext`, `Path<T>`, etc.).
///
/// # Example
///
/// ```rust,ignore
/// use craken_macros::{controller, delete, get, post};
///
/// pub struct UserController;
///
/// #[controller]
/// impl UserController {
///     #[get("/users")]
///     pub async fn index(
///         Inject(svc): Inject<UserService>,
///     ) -> Result<Json<Vec<User>>, CrakenError> {
///         Ok(Json(svc.all()))
///     }
///
///     #[get("/users/:id")]
///     pub async fn show(
///         Path(id): Path<u64>,
///         ctx: RequestContext,
///     ) -> Result<Json<User>, CrakenError> {
///         ctx.resolve::<UserService>()
///             .ok_or_else(|| CrakenError::internal("missing"))?
///             .find(id)
///             .map(Json)
///             .ok_or_else(|| CrakenError::NotFound(format!("user {id} not found")))
///     }
///
///     #[post("/users")]
///     pub async fn create() -> axum::http::StatusCode {
///         axum::http::StatusCode::CREATED
///     }
///
///     #[delete("/users/:id")]
///     pub async fn destroy(Path(id): Path<u64>) -> axum::http::StatusCode {
///         axum::http::StatusCode::NO_CONTENT
///     }
/// }
///
/// // Equivalent generated code (simplified):
/// //
/// // impl craken_http::RouteProvider for UserController {
/// //     fn routes(&self) -> axum::Router<Arc<Container>> {
/// //         axum::Router::new()
/// //             .route("/users",     axum::routing::get(UserController::index))
/// //             .route("/users/:id", axum::routing::get(UserController::show))
/// //             .route("/users",     axum::routing::post(UserController::create))
/// //             .route("/users/:id", axum::routing::delete(UserController::destroy))
/// //     }
/// // }
/// ```
#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    controller::expand(args, input)
}

/// Derive macro for [`craken_database::model::Model`].
///
/// Automatically implements `table_name()` by converting the struct name
/// to snake_case. Use `#[table("custom_name")]` to override.
#[proc_macro_derive(Model, attributes(table))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    model::expand(input)
}
