use crate::{
    error::PetsError, pet_to_trainer::pet_to_trainer, trainer_to_pets::trainer_to_pets,
    AuthedContext, Database,
};
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Trainer {
    pub name: String,
    pub cash: i32,
}

#[juniper::graphql_object(
    Context = AuthedContext,
)]
impl Trainer {
    fn name(&self) -> &str {
        &self.name
    }

    fn cash(&self) -> i32 {
        (&self).cash
    }

    fn pets(&self, context: &AuthedContext) -> Vec<Pet> {
        match context.database.trainers.get(&self.name) {
            Ok(Some(trainer)) => trainer_to_pets(&context.database, &trainer),
            _ => vec![],
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TrainerInternal {
    pub name: String,
    pub cash: i32,
    pub pet_ids: Vec<String>,
    pub password: String,
    pub last_seen: u64,
}

impl Into<Trainer> for TrainerInternal {
    fn into(self) -> Trainer {
        Trainer {
            name: self.name,
            cash: self.cash,
        }
    }
}

impl TrainerInternal {
    pub fn pay(&mut self, database: &Database, amount: i32) -> Result<(), PetsError> {
        if amount > self.cash {
            return Err(PetsError);
        }

        self.cash -= amount;

        database.trainers.insert(&*self.name, self.clone())?;

        Ok(())
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
    Context = AuthedContext,
)]
impl Pet {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn trainer(&self, context: &AuthedContext) -> Option<Trainer> {
        match context.database.pets.get(&self.id) {
            Ok(Some(pet)) => pet_to_trainer(&context.database, &pet),
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

#[derive(Clone, Deserialize, Serialize)]
pub struct PetInternal {
    pub id: String,
    pub name: String,
    pub trainer_name: Option<String>,
    pub species: Species,
    pub art_seed: i32,
    pub age: f64,
    pub last_fed: u64,
}

impl Into<Pet> for PetInternal {
    fn into(self) -> Pet {
        Pet {
            id: self.id,
            name: self.name,
            species: self.species,
            age: (self.age as i32),
            last_fed: (self.last_fed as i32),
            art_seed: self.art_seed,
        }
    }
}

impl From<&PetInternal> for Pet {
    fn from(pet: &PetInternal) -> Pet {
        Pet {
            id: pet.id.clone(),
            name: pet.name.clone(),
            species: pet.species.clone(),
            age: (pet.age as i32),
            last_fed: (pet.last_fed as i32),
            art_seed: pet.art_seed,
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

#[derive(GraphQLObject)]
pub struct UpdateResult {
    pub duration: i32,
    pub pets: Vec<UpdatePet>,
}

#[derive(GraphQLObject)]
pub struct UpdatePet {
    pub pet_id: String,
    pub from_age: f64,
    pub to_age: f64,
    pub cash_earned: i32,
}
