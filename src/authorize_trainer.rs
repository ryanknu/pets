use crate::{types::AuthorizeResult, Database};
/// Authorizes a trainer
///
pub fn authorize_trainer(sled: &Database, username: &str, password: &str) -> AuthorizeResult {
    // TODO: store password stronger with either bcrypt or argon.
    let result = sled.trainers.get(username);
    if let Ok(Some(result)) = result {
        if result.password.eq(password) {
            return AuthorizeResult {
                success: false,
                jwt: String::from(username),
            };
        }
    }
    AuthorizeResult {
        success: false,
        jwt: String::from(""),
    }
}
