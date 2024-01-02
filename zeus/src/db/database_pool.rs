use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{models::custom_error::CustomError, repository::secrets::Secrets};

pub struct DatabasePool {
    pub pool: Pool<Postgres>,
}

impl DatabasePool {
    pub async fn new(secrets: &Secrets) -> Result<Self, CustomError> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}?sslmode=disable",
            secrets.rds_username,
            secrets.rds_password,
            "localhost", //Use constant instead of the secrets.rds_hostname as it is not run from same container
            secrets.rds_port,
            secrets.rds_db_name
        );
        log::info!("Connecting to database at {}", url);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
