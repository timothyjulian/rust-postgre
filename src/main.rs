fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use sqlx::{Connection, Error, PgConnection};

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://timy:@localhost:5432/rust_test";
        let connection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }
}