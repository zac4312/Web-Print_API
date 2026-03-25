use Web_Print_API::db;

#[tokio::test]
async fn test_pool() -> Result<(), sqlx::Error>{ db::connect().await?; Ok(())}



