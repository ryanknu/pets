use crate::types::{PetInternal, TrainerInternal};
use juniper::{EmptySubscription, RootNode};
use mutation::Mutation;
use query::Query;
use rocket::State;
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;

mod adopt_pet;
mod adoptable_pets;
mod auth;
mod authorize_trainer;
mod create_trainer;
mod feed_pet;
mod mutation;
mod pet_to_trainer;
mod query;
mod trainer;
mod trainer_to_pets;
mod types;
mod update;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Sled Connections
///
/// This is implemented as Context (one read-only copy for all requests)
/// because (I think) sled is fairly single-threaded. In a higher-
/// throughput application, you would store a DB connection pool in Context
/// and vend a connection from that pool to each Request using a FromRequest
/// deserializer.
pub struct Database {
    adoptable_pets: Tree<Vec<PetInternal>>,
    pets: Tree<PetInternal>,
    trainers: Tree<TrainerInternal>,
}

pub struct AuthedContext {
    pub auth_key: String,
    pub database: Database,
}

impl juniper::Context for AuthedContext {}

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<AuthedContext>>;

fn make_context(database: &Database, auth_key: String) -> AuthedContext {
    AuthedContext {
        auth_key: auth_key,
        database: Database {
            adoptable_pets: database.adoptable_pets.clone(),
            pets: database.pets.clone(),
            trainers: database.trainers.clone(),
        },
    }
}

#[rocket::get("/api?<request>")]
fn api_get(
    database: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    key: auth::ApiKey,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &make_context(database, key.0.to_string()))
}

#[rocket::post("/api", data = "<request>")]
fn api_post(
    database: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    key: auth::ApiKey,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &make_context(database, key.0.to_string()))
}

#[rocket::main]
async fn main() {
    let db = sled_extensions::Config::default()
        .path("./sled_data")
        .open()
        .expect("Failed to open sled db");

    rocket::build()
        .manage(Database {
            adoptable_pets: db
                .open_bincode_tree("adoptable_pets")
                .expect("failed to open adoptable_pets tree"),
            pets: db
                .open_bincode_tree("pets")
                .expect("failed to open adoptable_pets tree"),
            trainers: db
                .open_bincode_tree("trainers")
                .expect("failed to open trainers tree"),
        })
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<AuthedContext>::new(),
        ))
        .mount("/", rocket::routes![api_get, api_post])
        .launch()
        .await
        .expect("it to blast off")
}
