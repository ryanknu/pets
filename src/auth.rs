use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct ApiKey<'r>(pub &'r str);

#[derive(Debug)]
pub enum ApiKeyError {}

#[derive(Serialize, Deserialize)]
struct JwtHeader {
    alg: String,
    typ: String,
}

#[derive(Serialize, Deserialize)]
struct JwtPayload {
    username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(ApiKey(req.headers().get_one("authorization").unwrap_or("")))
    }
}

// pub fn make_jwt(username: String) -> String {
//     let header = r#"{"alg":"HS256","typ":"jwt"}"#;
//     let payload = serde_json::to_string(&JwtPayload { username: username })
//         .unwrap_or(String::from(r#"{"username":""}"#));
//     let signature = "abc";
//     format!("{}.{}.{}", header, payload, signature)
// }

// pub fn validate_basic_auth(header: &str) -> &str {
//     "ryan"
// }

pub fn time() -> u64 {
    let now = SystemTime::now();
    let now = now.duration_since(UNIX_EPOCH);
    let now = now.unwrap_or(Duration::from_secs(0));
    now.as_secs()
}
