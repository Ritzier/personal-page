use std::env;

use dotenvy::dotenv;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use super::error::DatabaseError;

pub struct Database(pub Surreal<Client>);

impl Database {
    /// Get `DB_USER`, `DB_PASS`, `DB_PORT` from `.env` file
    /// Sigin root with `DB_USER` and `DB_PASS`
    pub async fn init() -> Result<Self, DatabaseError> {
        dotenv()?;

        let db_user = env::var("DB_USER")?;
        let db_pass = env::var("DB_PASS")?;
        let db_port = env::var("DB_PORT")?;

        let database = Surreal::new::<Ws>(&format!("127.0.0.1:{}", db_port)).await?;

        database
            .signin(Root {
                username: &db_user,
                password: &db_pass,
            })
            .await?;

        Ok(Self(database))
    }
}
