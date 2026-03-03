use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitStr};

/// Expand a standalone route attribute on a free async function.
///
/// Emits the original function unchanged **plus** a zero-sized companion struct
/// `{FnName}Route` that implements `craken_http::RouteProvider`, so the
/// handler can be mounted with `HttpServer::configure_routes(&FnNameRoute)`.
///
/// # Name derivation
///
/// `list_users` → `ListUsersRoute`
pub fn expand(method: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(args as LitStr);
    let func = parse_macro_input!(input as ItemFn);

    let fn_ident = &func.sig.ident;
    let vis = &func.vis;

    let struct_ident = format_ident!("{}Route", snake_to_pascal(&fn_ident.to_string()));
    let method_ident = format_ident!("{}", method);

    let expanded = quote! {
        // Original function — untouched.
        #func

        /// Auto-generated [`craken_http::RouteProvider`] for `#fn_ident`.
        ///
        /// Mount via:
        /// ```rust,ignore
        /// HttpServer::new().configure_routes(&#struct_ident)
        /// ```
        #[derive(Default)]
        #vis struct #struct_ident;

        impl ::craken_http::RouteProvider for #struct_ident {
            fn routes(
                &self,
            ) -> ::axum::Router<::std::sync::Arc<::craken_container::Container>> {
                ::axum::Router::new()
                    .route(#path_lit, ::axum::routing::#method_ident(#fn_ident))
            }
        }
    };

    expanded.into()
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// `list_users` → `ListUsers`
fn snake_to_pascal(snake: &str) -> String {
    snake
        .split('_')
        .map(|seg| {
            let mut chars = seg.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
