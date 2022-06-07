use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{RegisteredClaims, VerifyWithKey};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::{Request, request};
use rocket::request::FromRequest;
use sha2::{Sha256};

pub struct ApiKey(pub String);

pub fn read_token(key: &str) -> Result<String, String> {
    let algo: Hmac<Sha256> = Hmac::new_from_slice("secret".as_bytes()).unwrap();
    let claims: RegisteredClaims = VerifyWithKey::verify_with_key(key, &algo).map_err(|e| e.to_string())?;
    claims.subject.ok_or("No subject".to_string())
}

#[rocket::async_trait]
impl <'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(token) => Outcome::Success(ApiKey(token)),
            Err(_) => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}
