#[derive(Debug, PartialEq)]
pub enum ServiceErr {
    VendorNotFound,
    VendorUnavailable,
    DatabaseError
}

impl From<sqlx::Error> for ServiceErr {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("Database Error: {:?}", err);
        ServiceErr::DatabaseError
    }
}
