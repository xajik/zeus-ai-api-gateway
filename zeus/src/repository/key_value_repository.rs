use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{Pool, Postgres, Row};

use crate::db::database_pool::DatabasePool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct KeyValueEntity {
    id: i32,
    json_body: JsonValue,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct KeyValueRepository {
    db: Arc<DatabasePool>,
}

impl KeyValueRepository {
    pub fn new(db: Arc<DatabasePool>) -> Self {
        Self { db }
    }

    pub async fn fetch_one(&self, entity_id: i32) -> sqlx::Result<KeyValueEntity> {
        let entity = sqlx::query_as::<_, KeyValueEntity>(
            "SELECT id, json_body, created_at, updated_at FROM key_value_store WHERE id = $1",
        )
        .bind(entity_id)
        .fetch_one(self.db.pool())
        .await?;

        Ok(entity)
    }

    pub async fn fetch_many(&self) -> sqlx::Result<Vec<KeyValueEntity>> {
        let entities = sqlx::query_as::<_, KeyValueEntity>(
            "SELECT id, json_body, created_at, updated_at FROM key_value_store",
        )
        .fetch_all(self.db.pool())
        .await?;

        Ok(entities)
    }

    pub async fn insert_one(&self, json_body: JsonValue) -> Result<i32, sqlx::Error> {
        let row = sqlx::query("INSERT INTO key_value_store (json_body) VALUES ($1) RETURNING id")
            .bind(json_body)
            .fetch_one(self.db.pool())
            .await?;
        let id: i32 = row.try_get(0)?;

        Ok(id)
    }

    pub async fn insert_many(&self, json_bodies: Vec<JsonValue>) -> Result<Vec<i32>, sqlx::Error> {
        let mut ids = Vec::new();

        let query_str = "INSERT INTO key_value_store (json_body) VALUES ($1) RETURNING id";
        for json_body in json_bodies {
            let row = sqlx::query(query_str)
                .bind(json_body)
                .fetch_one(self.db.pool())
                .await?;
            let id: i32 = row.try_get(0)?;
            ids.push(id);
        }
        Ok(ids)
    }
}
