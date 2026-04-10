

#[derive(Debug, PartialEq)]
pub enum TransactionErr {
    VendorNotFound,
    VendorUnavailable,    
    TransactionDatabaseError
}

impl From<sqlx::Error> for TransactionErr {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("Database Error: {:?}", err);
        TransactionErr::TransactionDatabaseError
    }
}

#[derive(PartialEq, Debug)]
pub enum VendorErr {
   OrderNotFoud,
   VendorDatabaseErr
}

impl From<sqlx::Error> for VendorErr {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("Database Error: {:?}", err);
        VendorErr::VendorDatabaseErr
    }
}
