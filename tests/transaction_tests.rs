use Web_Print_API::{db, service::{transaction}, err::ServiceErr};

#[tokio::test]
    async fn choose_vendor_not_exist_test() -> Result<(), ServiceErr> {
        let input = "non-existant ID".to_string(); let con = db::connect().await?;                 
        let result = transaction::choose_vendor(&con, &input).await;

        assert_eq!(result.unwrap_err(), ServiceErr::VendorNotFound);

        Ok(())
    }

#[tokio::test]
    async fn choose_vendor_is_closed() -> Result<(), ServiceErr> {
        let input = "pub_test_001".to_string(); let con = db::connect().await?;                 
        let result = transaction::choose_vendor(&con, &input).await;

        assert_eq!(result.unwrap_err(), ServiceErr::VendorNotFound);

        Ok(())
    }




