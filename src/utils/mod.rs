use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode, errors::ErrorKind};
use rand::{RngExt, distr::Alphanumeric};
use http::{HeaderMap};
use crate::dto::jwt::Claims;
 
pub fn generate_id(n: usize) -> String{
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .map(char::from)
            .collect()
}

pub fn validate_token(token: String) -> Result<TokenData<Claims>, ErrorKind> {
    let validation = Validation::new(Algorithm::HS256);
    let secret = b"secret";

    let token_data: TokenData<Claims>= decode(token, &DecodingKey::from_secret(secret), &validation).unwrap();    

    Ok(token_data)
}

pub fn get_token(header: HeaderMap) -> Result<String, http::Error> {
    let auth_header = header.get("authorization"); let auth_string = auth_header.unwrap().to_str().unwrap(); 
    let token = auth_string.trim_start_matches("Bearer ").to_string();

    Ok(token)
}
