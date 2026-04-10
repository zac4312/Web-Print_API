use Web_Print_API::{db, err::TransactionErr, models::transaction_obj::File, service::transaction::attach_file};
use uuid::Uuid;

#[tokio::test]
async fn create_file_test() -> Result<(), TransactionErr> { 
   
    let con = db::connect().await?;
    let mut tx = con.begin().await?;

    let file_path = "/home/yser/test".to_string(); let file_size = 1200; let mime_type = "jpg".to_string(); let deleted_at = None; 
    let file = File::new(file_path, file_size, mime_type, deleted_at);

    let result = attach_file(&mut tx, file).await?;
    let expected = Uuid::new_v4();

    assert_ne!(result, expected);

Ok(())

}
