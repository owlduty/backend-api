use super::JWT_SECRET;
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct Claims {
    role: String,
    user_id: String,
    exp: usize,
}

pub(crate) fn jwt_is_valid(token: &str) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
}
