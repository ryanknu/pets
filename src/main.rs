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

impl juniper::Context for Database {}

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

#[rocket::get("/api?<request>")]
fn api_get(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    key: auth::ApiKey,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::post("/api", data = "<request>")]
fn api_post(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    key: auth::ApiKey,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
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
            EmptySubscription::<Database>::new(),
        ))
        .mount("/", rocket::routes![api_get, api_post])
        .launch()
        .await
        .expect("it to blast off")
}
