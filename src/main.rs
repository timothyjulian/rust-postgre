mod result_mapping;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::TryStreamExt;
    use sqlx::{Connection, Error, PgConnection, Pool, Postgres, Row, postgres::PgPoolOptions};
    use uuid::Uuid;

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

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://timy:@localhost:5432/rust_test";
        let connection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;
        let my_uuid = Uuid::new_v4();
        sqlx::query(&format!(
            "INSERT INTO category(id, name, description) VALUES('{}', 'Sample', 'Sample');",
            my_uuid
        ))
        .execute(&pool)
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;
        let my_uuid = Uuid::new_v4();
        sqlx::query("INSERT INTO category(id, name, description) VALUES($1, $2, $3);")
            .bind(my_uuid.to_string())
            .bind("Sample name")
            .bind("Sample description")
            .execute(&pool)
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("A")
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        } else {
            println!("data is not found");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_one() -> Result<(), Error> {
        // will throw if not found
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("A")
            .fetch_one(&pool)
            .await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");

        println!("id: {}, name: {}, description: {}", id, name, description);

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("C")
            .fetch_all(&pool)
            .await?;

        for row in result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");

            println!("id: {}, name: {}, description: {}", id, name, description);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_stream() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut result = sqlx::query("SELECT * FROM category").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        }

        Ok(())
    }
}
