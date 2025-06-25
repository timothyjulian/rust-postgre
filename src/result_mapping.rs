use std::time::Duration;

use sqlx::{Error, FromRow, Pool, Postgres, postgres::PgPoolOptions};

#[derive(FromRow, Debug)]
struct Category {
    id: String,
    name: String,
    description: String,
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
    use std::result;

    use sqlx::{Error, Row, postgres::PgRow};

    use crate::result_mapping::{Category, get_pool};

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
}