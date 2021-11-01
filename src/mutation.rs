use crate::{
    auth::time,
    types::{AuthorizeResult, Trainer, UpdateResult},
    AuthedContext,
};
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

pub struct Mutation;

#[graphql_object(
    Context = AuthedContext,
)]
impl Mutation {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn adopt_pet(
        context: &AuthedContext,
        pet_id: String,
        pet_name: String,
    ) -> FieldResult<Trainer> {
        match crate::adopt_pet::adopt_pet(
            &context.database,
            &*context.auth_key,
            &*pet_id,
            &*pet_name,
        ) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(trainer),
        }
    }

    fn authorize_trainer(
        context: &AuthedContext,
        username: String,
        password: String,
    ) -> AuthorizeResult {
        crate::authorize_trainer::authorize_trainer(&context.database, &*username, &*password)
    }

    fn create_trainer(
        context: &AuthedContext,
        username: String,
        password: String,
    ) -> FieldResult<AuthorizeResult> {
        match crate::create_trainer::create_trainer(&context.database, &*username, &*password) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(_) => FieldResult::Ok(AuthorizeResult {
                success: true,
                jwt: username,
            }),
        }
    }

    fn feed_pet(context: &AuthedContext, pet_id: String) -> FieldResult<Trainer> {
        match crate::feed_pet::feed_pet(&context.database, &*context.auth_key, &*pet_id) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(trainer),
        }
    }

    fn update(context: &AuthedContext) -> FieldResult<UpdateResult> {
        match crate::update::update(&context, context.auth_key.to_string(), time()) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(result) => FieldResult::Ok(result),
        }
    }
}
