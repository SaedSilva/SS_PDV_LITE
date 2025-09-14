use sqlx::SqlitePool;

struct ProductRepository {
    pool: SqlitePool
}