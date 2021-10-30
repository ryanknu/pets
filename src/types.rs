use crate::{pet_to_trainer::pet_to_trainer, trainer_to_pets::trainer_to_pets, Database};
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Trainer {
    pub name: String,
    pub cash: i32,
}

#[juniper::graphql_object(
    Context = Database,
)]
impl Trainer {
    fn name(&self) -> &str {
        &self.name
    }

    fn cash(&self) -> i32 {
        (&self).cash
    }

    fn pets(&self, database: &Database) -> Vec<Pet> {
        match database.trainers.get(&self.name) {
            Ok(Some(trainer)) => trainer_to_pets(database, trainer.pet_ids),
            _ => vec![],
        }
    }
}

#[derive(Clone, Deserialize, GraphQLObject, Serialize)]
pub struct TrainerInternal {
    pub name: String,
    pub cash: i32,
    pub pet_ids: Vec<String>,
    pub password: String,
}

impl Into<Trainer> for TrainerInternal {
    fn into(self) -> Trainer {
        Trainer {
            name: self.name,
            cash: self.cash,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Pet {
    pub id: String,
    pub name: String,
    pub species: Species,
    pub art_seed: i32,
    pub age: i32,
    pub last_fed: i32,
}

#[juniper::graphql_object(
    Context = Database,
)]
impl Pet {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn trainer(&self, database: &Database) -> Option<Trainer> {
        match database.pets.get(&self.id) {
            Ok(Some(pet)) => match pet.trainer_name {
                Some(trainer_name) => pet_to_trainer(database, trainer_name),
                None => None,
            },
            _ => None,
        }
    }

    fn species(&self) -> Species {
        (&self.species).clone()
    }

    fn art_seed(&self) -> i32 {
        (&self).art_seed
    }

    fn age(&self) -> i32 {
        (&self).age
    }

    fn last_fed(&self) -> i32 {
        (&self).last_fed
    }
}

#[derive(Clone, Deserialize, GraphQLObject, Serialize)]
pub struct PetInternal {
    pub id: String,
    pub name: String,
    pub trainer_name: Option<String>,
    pub species: Species,
    pub art_seed: i32,
    pub age: i32,
    pub last_fed: i32,
}

impl Into<Pet> for PetInternal {
    fn into(self) -> Pet {
        Pet {
            id: self.id,
            name: self.name,
            species: self.species,
            age: self.age,
            last_fed: self.last_fed,
            art_seed: self.art_seed,
        }
    }
}

#[derive(Clone, Deserialize, GraphQLEnum, Serialize)]
pub enum Species {
    Flarf,
    Glabor,
    Tiqqa,
}

#[derive(GraphQLObject)]
pub struct AuthorizeResult {
    pub success: bool,
    pub jwt: String,
}