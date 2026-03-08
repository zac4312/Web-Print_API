use rand::{RngExt, distr::Alphanumeric};
 
pub fn generate_id(n: usize) -> String{
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .map(char::from)
            .collect()
}
