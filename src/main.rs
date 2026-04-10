use Web_Print_API::{routes::{listener, route}}; 

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{

    let api = route(); let listener = listener().await;
    axum::serve(listener, api).await.expect("err");

    Ok(())
}
