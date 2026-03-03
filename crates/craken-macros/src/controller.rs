use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItem, ItemImpl, LitStr};

// HTTP verbs recognised as routing attributes inside a `#[controller]` impl.
const ROUTE_VERBS: &[&str] = &["get", "post", "put", "delete", "patch"];

struct RouteEntry {
    /// `get`, `post`, …
    verb: String,
    /// `/users`, `/users/:id`, …
    path: String,
    /// The method's identifier.
    fn_ident: syn::Ident,
}

/// Core expansion logic for `#[controller]`.
///
/// Steps
/// 1. Parse the impl block with `syn`.
/// 2. Walk every `fn` item; collect and **strip** route attributes.
/// 3. Strip any `self` / `&self` / `&mut self` receiver from annotated
///    methods so axum can call them as plain async functions.
/// 4. Emit the cleaned impl block followed by a generated
///    `RouteProvider` impl.
pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    let mut routes: Vec<RouteEntry> = Vec::new();

    // ── Walk methods ──────────────────────────────────────────────────────────
    for item in &mut impl_block.items {
        let ImplItem::Fn(method) = item else {
            continue;
        };

        // Collect route attrs, removing them so they don't appear in output.
        let mut found: Vec<(String, String)> = Vec::new();

        method.attrs.retain(|attr| {
            let Some(ident) = attr.path().get_ident() else {
                return true; // keep unknown attrs
            };
            let verb = ident.to_string();
            if ROUTE_VERBS.contains(&verb.as_str()) {
                if let Ok(path_lit) = attr.parse_args::<LitStr>() {
                    found.push((verb, path_lit.value()));
                }
                false // strip the attribute
            } else {
                true
            }
        });

        if found.is_empty() {
            continue; // plain method — leave untouched
        }

        // Strip `self` / `&self` / `&mut self` receiver.
        // Controller methods obtain state entirely through axum extractors.
        if let Some(FnArg::Receiver(_)) = method.sig.inputs.first() {
            let rest: syn::punctuated::Punctuated<FnArg, syn::Token![,]> =
                method.sig.inputs.iter().skip(1).cloned().collect();
            method.sig.inputs = rest;
        }

        for (verb, path) in found {
            routes.push(RouteEntry {
                verb,
                path,
                fn_ident: method.sig.ident.clone(),
            });
        }
    }

    // ── Generate RouteProvider impl ───────────────────────────────────────────
    let self_ty = &impl_block.self_ty;

    let registrations = routes.iter().map(|r| {
        let verb = syn::Ident::new(&r.verb, proc_macro2::Span::call_site());
        let path = &r.path;
        let fn_ident = &r.fn_ident;
        quote! {
            .route(#path, ::axum::routing::#verb(#self_ty::#fn_ident))
        }
    });

    let expanded = quote! {
        // Cleaned impl block — route attrs and self receivers are gone.
        #impl_block

        impl ::craken_http::RouteProvider for #self_ty {
            fn routes(
                &self,
            ) -> ::axum::Router<::std::sync::Arc<::craken_container::Container>> {
                ::axum::Router::new()
                    #(#registrations)*
            }
        }
    };

    expanded.into()
}
