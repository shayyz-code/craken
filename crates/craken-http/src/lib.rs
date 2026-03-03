pub mod context;
pub mod error;
pub mod inject;
pub mod middleware;
pub mod server;

// ── Flat re-exports for ergonomic imports ─────────────────────────────────────

pub use context::RequestContext;
pub use error::CrakenError;
pub use inject::Inject;
pub use middleware::{CrakenMiddleware, LoggingMiddleware, MiddlewareStack};
pub use server::{HttpServer, RouteProvider};

// ── Crate re-exports used by `craken-macros` generated code ───────────────────
//
// Macro-generated `RouteProvider` impls reference `::axum::Router`,
// `::axum::routing::{get,post,…}`, and `::craken_container::Container`.
// Re-exporting them here means application crates only need `craken-http`
// as a direct dependency for the generated code to compile.

pub use axum;
pub use craken_container;
