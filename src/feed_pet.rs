use crate::{types::Trainer, Database};
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn feed_pet<'a>(database: &Database, username: &str, pet_id: &str) -> Result<Trainer, &'a str> {
    let result = database.trainers.get(username);
    if let Ok(Some(trainer)) = result {
        if trainer.cash < 50 {
            return Err("Not enough cash to feed pet :(");
        }

        let new_fed_time = SystemTime::now();
        let new_fed_time = new_fed_time.duration_since(UNIX_EPOCH);

        if let Err(_) = new_fed_time {
            return Err("An internal error occurred");
        }

        let new_fed_time = new_fed_time.unwrap();

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
            // TODO: crashes when last_fed is > 2038. should change to use Chrono
            updated_pet.last_fed = i32::try_from(new_fed_time.as_secs()).unwrap();
            database.pets.insert(pet_id, updated_pet);
            return Ok(updated_trainer.into());
        }
    }
    Err("Trainer not found")
}
