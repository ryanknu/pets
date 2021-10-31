use rocket::request::{FromRequest, Outcome, Request};

pub struct ApiKey<'r>(pub &'r str);

#[derive(Debug)]
pub enum ApiKeyError {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(ApiKey(req.headers().get_one("authorization").unwrap_or("")))
    }
}

pub fn validate_basic_auth(header: &str) -> &str {
    "ryan"
}
