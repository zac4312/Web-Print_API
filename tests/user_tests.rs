use Web_Print_API::{models::users::User, service::user::{create_user, get_users}, db};

#[tokio::test]
    async fn create_user_test() -> Result<(), sqlx::Error> {
        let con = db::connect().await?;

        let name = "TEST"; let pw = "TEST"; let email = "a@gmail.com"; 
        let user = User::new(name.to_string(), pw.to_string(), email.to_string());
        
        create_user(&con, user).await?;    
        Ok(())
}

#[tokio::test]
 async fn list_users_test() -> Result<(), sqlx::Error> {
    let con = db::connect().await?;

    get_users(&con).await?;
    Ok(())
 }

