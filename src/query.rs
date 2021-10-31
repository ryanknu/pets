use crate::{
    types::{Pet, Trainer},
    AuthedContext,
};
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

pub struct Query;

#[graphql_object(
    Context = AuthedContext,
)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn adoptable_pets(context: &AuthedContext) -> Vec<Pet> {
        crate::adoptable_pets::adoptable_pets(&context.database, &*context.auth_key)
    }

    fn trainer(context: &AuthedContext) -> FieldResult<Trainer> {
        match crate::trainer::trainer(&context.database, &*context.auth_key) {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(trainer),
        }
    }
}
