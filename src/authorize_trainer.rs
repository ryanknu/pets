use crate::{types::AuthorizeResult, Database};
/// Authorizes a trainer
///
pub fn authorize_trainer(sled: &Database, username: &str, password: &str) -> AuthorizeResult {
    // TODO: check password
    // TODO: level up pets
    // TODO: add PetDelta to AuthorizeResult, which has From : To
    AuthorizeResult {
        success: false,
        jwt: "".into(),
    }
}
