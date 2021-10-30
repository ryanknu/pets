use crate::{
    types::{AuthorizeResult, Trainer},
    Database,
};
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

pub struct Mutation;

#[graphql_object(
    Context = Database,
)]
impl Mutation {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn adopt_pet(context: &Database, pet_id: String, pet_name: String) -> FieldResult<Trainer> {
        match crate::adopt_pet::adopt_pet(context, "ryan", &*pet_id, &*pet_name) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(trainer),
        }
    }

    fn authorize_trainer(
        context: &Database,
        username: String,
        password: String,
    ) -> AuthorizeResult {
        crate::authorize_trainer::authorize_trainer(context, &*username, &*password)
    }

    fn create_trainer(
        context: &Database,
        username: String,
        password: String,
    ) -> FieldResult<AuthorizeResult> {
        match crate::create_trainer::create_trainer(context, &*username, &*password) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(AuthorizeResult {
                success: true,
                jwt: "".into(),
            }),
        }
    }

    fn feed_pet(context: &Database, pet_id: String) -> FieldResult<Trainer> {
        match crate::feed_pet::feed_pet(context, "ryan", &*pet_id) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(trainer),
        }
    }
}
