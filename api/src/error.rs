use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Env Error: {0:?}")]
    EnvError(#[from] env::VarError),

    #[error("Dotenvy Error: {0:?}")]
    DotEnvyError(#[from] dotenvy::Error),

    #[error("Surreal Error: {0:?}")]
    SurrealError(#[from] surrealdb::Error),
}
