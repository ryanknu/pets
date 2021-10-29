use crate::{Database, types::{Pet, Trainer}};
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

pub struct Query;

#[graphql_object(
    Context = Database,
)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn adoptable_pets(context: &Database) -> Vec<Pet> {
        crate::adoptable_pets::adoptable_pets(context, "ryan")
    }

    fn trainer(context: &Database) -> FieldResult<Trainer> {
        match crate::trainer::trainer(context, "ryan") {
            Err(str) => FieldResult::Err(FieldError::new(
                str,
                graphql_value!({ "internal_error": str }),
            )),
            Ok(trainer) => FieldResult::Ok(trainer),
        }
    }
}
