use crate::{
    types::{PetInternal, Trainer},
    Database,
};

pub fn pet_to_trainer(database: &Database, pet: &PetInternal) -> Option<Trainer> {
    match &pet.trainer_name {
        Some(trainer_name) => match database.trainers.get(trainer_name) {
            Ok(Some(trainer)) => Some(trainer.into()),
            _ => None,
        },
        _ => None,
    }
}
