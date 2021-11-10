use crate::{
    types::{Pet, TrainerInternal},
    Database,
};

pub fn trainer_to_pets(database: &Database, trainer: &TrainerInternal) -> Vec<Pet> {
    trainer
        .pet_ids
        .iter()
        .filter_map(|x| get_pet(database, x))
        .collect()
}

fn get_pet(database: &Database, pet_id: &String) -> Option<Pet> {
    if let Ok(Some(pet)) = database.pets.get(pet_id) {
        Some(pet.into())
    } else {
        None
    }
}
