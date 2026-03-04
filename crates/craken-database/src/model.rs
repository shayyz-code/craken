/// Core Model trait for ORM-like functionality.
///
/// The `#[derive(Model)]` macro will provide the default implementations
/// for `table_name` and standard CRUD operations.
pub trait Model: Send + Sync + Unpin + 'static
    + for<'r> ::sqlx::FromRow<'r, ::sqlx::postgres::PgRow>
    + for<'r> ::sqlx::FromRow<'r, ::sqlx::sqlite::SqliteRow>
    + ::serde::Serialize
{
    /// The database table name for this model.
    fn table_name() -> &'static str;

    /// Primary key column name (defaults to "id").
    fn primary_key() -> &'static str {
        "id"
    }
}
