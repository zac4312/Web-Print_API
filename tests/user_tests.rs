use Web_Print_API::{db::{self, connect}, err::TransactionErr, models::users::User, service::user::{create_user} };
use uuid::Uuid;

#[tokio::test]
async fn create_user_test() -> Result<(), TransactionErr> {
    let con = connect().await?;
    let name = "name_test"; let pw_hash = "pwd_test"; let email = "test@email.com";
    let user = User::new(name.to_string(), pw_hash.to_string(), email.to_string ());

    create_user(&con, user).await?; 

    Ok(())
}

