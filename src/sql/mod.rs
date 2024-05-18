pub mod methods;
pub mod models;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct DB {
    pool: Pool<Postgres>,
}
