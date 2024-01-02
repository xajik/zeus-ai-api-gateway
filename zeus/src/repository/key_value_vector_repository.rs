use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool, Row};

use crate::{db::database_pool::DatabasePool, models::custom_error::CustomError};

#[derive(Debug, FromRow)]
pub struct KeyValueVectorEntity {
    id: i32,
    vector_data: Vec<f32>,
    metadata: JsonValue,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct KeyValueVectorRepository {
    db: Arc<DatabasePool>,
}

impl KeyValueVectorRepository {
    pub fn new(pool: Arc<DatabasePool>) -> Self {
        Self { db: pool }
    }

    pub async fn insert_one(
        &self,
        vector_data: Vec<f32>,
        metadata: JsonValue,
    ) -> Result<i32, CustomError> {
        let query_str =
            "INSERT INTO key_value_vector (vector_data, metadata) VALUES ($1, $2) RETURNING id";
        let row = sqlx::query(query_str)
            .bind(vector_data)
            .bind(metadata)
            .fetch_one(self.db.pool())
            .await?;

        let id: i32 = row.try_get(0)?;
        Ok(id)
    }

    pub async fn search_by_distance(
        &self,
        vector: Vec<f32>,
        limit: i64,
    ) -> Result<Vec<KeyValueVectorEntity>, CustomError> {
        let query_str = "SELECT * FROM key_value_vector ORDER BY vector_data <-> $1 LIMIT $2";
        let rows = sqlx::query_as::<_, KeyValueVectorEntity>(query_str)
            .bind(vector)
            .bind(limit)
            .fetch_all(self.db.pool())
            .await?;

        Ok(rows)
    }

    pub async fn find_nearest_neighbors(
        &self,
        reference_id: i32,
        limit: i64,
    ) -> Result<Vec<KeyValueVectorEntity>, CustomError> {
        let query_str = "
            SELECT * FROM key_value_vector 
            WHERE id != $1 
            ORDER BY vector_data <-> (
                SELECT vector_data FROM key_value_vector WHERE id = $1
            ) 
            LIMIT $2";

        let rows = sqlx::query_as::<_, KeyValueVectorEntity>(query_str)
            .bind(reference_id)
            .bind(limit)
            .fetch_all(self.db.pool())
            .await?;

        Ok(rows)
    }
}
