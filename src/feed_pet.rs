use crate::auth::time;
use crate::{types::Trainer, Database};

pub fn feed_pet<'a>(database: &Database, username: &str, pet_id: &str) -> Result<Trainer, &'a str> {
    let result = database.trainers.get(username);
    if let Ok(Some(trainer)) = result {
        if trainer.cash < 50 {
            return Err("Not enough cash to feed pet :(");
        }

        let new_fed_time = time();

        let mut updated_trainer = trainer.clone();
        updated_trainer.cash -= 50;
        let result = database.trainers.insert(username, updated_trainer.clone());
        if let Err(_) = result {
            return Err("An internal error occurred");
        }

        // TODO: oh man this lets you pay for pets that you don't own :) or non-existent pets
        let pet = database.pets.get(pet_id);
        if let Ok(Some(pet)) = pet {
            let mut updated_pet = pet.clone();
            updated_pet.last_fed = new_fed_time;
            return match database.pets.insert(pet_id, updated_pet) {
                Ok(Some(_)) => Ok(updated_trainer.into()),
                _ => Err("Error saving pet"),
            };
        }
    }
    Err("Trainer not found")
}
