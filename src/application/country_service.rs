use crate::prelude::*;
use crate::{
    DbPool,
    infrastructure::audit::geolocation::{Country, CountryDetails},
};

pub struct CountryService {
    db: DbPool,
}

impl CountryService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Country> {
        sqlx::query_as(r#"SELECT * FROM countries WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Country> {
        sqlx::query_as(r#"SELECT * FROM countries WHERE LOWER(name) = LOWER(?)"#)
            .bind(name)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Country> {
        sqlx::query_as(r#"SELECT * FROM countries WHERE LOWER(code) = LOWER(?)"#)
            .bind(code)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn search(&self, value: &str) -> Vec<Country> {
        let value = &format!("%{value}%");
        sqlx::query_as(
            r#"SELECT * FROM countries WHERE LOWER(name) LIKE LOWER(?) OR LOWER(code) LIKE LOWER(?) ORDER BY name ASC"#,
        )
        .bind(value)
        .bind(value)
        .fetch_all(self.db.as_ref())
        .await
        .unwrap_or_default()
    }

    pub async fn lock(&self, id: i64) -> Result<()> {
        let _ = sqlx::query(r#"UPDATE countries SET locked = 1 WHERE id = ?"#)
            .bind(id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }

    pub async fn unlock(&self, id: i64) -> Result<()> {
        let _ = sqlx::query(r#"UPDATE countries SET locked = 0 WHERE id = ?"#)
            .bind(id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }

    pub async fn create_or_get(&self, country: &CountryDetails) -> Result<Country> {
        let _ = sqlx::query(r#"INSERT INTO countries (name, code) VALUES (?, ?)"#)
            .bind(&country.name)
            .bind(&country.code)
            .fetch_one(self.db.as_ref())
            .await;
        sqlx::query_as(r#"SELECT * FROM countries WHERE code = ?"#)
            .bind(&country.code)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }
}
