use std::time::Duration;

use chrono::NaiveDateTime;
use sqlx::{Error, FromRow, Pool, Postgres, postgres::PgPoolOptions};

#[derive(FromRow, Debug)]
struct Category {
    id: String,
    name: String,
    description: String,
}

#[derive(FromRow, Debug)]
struct Brand {
    id: String,
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

async fn get_pool() -> Result<Pool<Postgres>, Error> {
    let url = "postgres://timy:@localhost:5432/rust_test";
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect(url)
        .await
}

#[cfg(test)]
mod test {

    use chrono::{DateTime, Local, Utc};
    use sqlx::{Error, Row, postgres::PgRow};
    use uuid::Uuid;

    use crate::result_mapping::{Brand, Category, get_pool};


    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category")
            .map(|row: PgRow| Category {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .fetch_all(&pool)
            .await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_auto_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result: Vec<Category> = sqlx::query_as("SELECT * FROM category")
            .fetch_all(&pool)
            .await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_brand() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("INSERT INTO brands(id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
        .bind("A")
        .bind("Contoh")
        .bind("Contoh Deskripsi")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(&pool).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_result_mapping_brand() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result: Vec<Brand> = sqlx::query_as("SELECT * FROM brands")
            .fetch_all(&pool)
            .await?;

        for brand in result {
            println!("{:?}", brand);
        }

        Ok(())
    }

}
