use Web_Print_API::{routes::{listener, route}}; 

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{

    dotenvy::dotenv().ok();

    let api = route(); let listener = listener().await;
    axum::serve(listener, api).await.expect("err");

    Ok(())
}
