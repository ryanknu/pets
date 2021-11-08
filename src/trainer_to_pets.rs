use crate::{
    types::{Pet, TrainerInternal},
    Database,
};

pub fn trainer_to_pets(database: &Database, trainer: &TrainerInternal) -> Vec<Pet> {
    // TODO: make this use .map().filter().collect()
    let mut result = Vec::new();
    for pet_id in trainer.pet_ids.iter() {
        if let Some(pet) = get_pet(database, pet_id) {
            result.insert(result.len(), pet);
        }
    }
    result
}

fn get_pet(database: &Database, pet_id: &String) -> Option<Pet> {
    if let Ok(Some(pet)) = database.pets.get(pet_id) {
        return Some(pet.into());
    }
    None
}
