use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct Store {
    pub pool: Pool<Postgres>
}

impl Store {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {pool}
    }

}