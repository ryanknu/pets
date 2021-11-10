use crate::auth::time;
use crate::error::PetsError;
use crate::{types::Trainer, Database};

pub fn feed_pet<'a>(
    database: &Database,
    username: &String,
    pet_id: &String,
) -> Result<Trainer, &'a str> {
    match feed_pet_internal(database, username, pet_id) {
        Ok(trainer) => Ok(trainer),
        _ => Err("Error feeding pet"),
    }
}

fn feed_pet_internal(
    database: &Database,
    username: &String,
    pet_id: &String,
) -> Result<Trainer, PetsError> {
    let mut trainer = database.trainers.get(username)?.ok_or(PetsError)?;
    trainer.pay(database, 50)?;

    if !trainer.pet_ids.iter().any(|id| pet_id == id) {
        return Err(PetsError);
    }

    let mut pet = database.pets.get(pet_id)?.ok_or(PetsError)?;

    pet.last_fed = time();
    database.pets.insert(&pet_id[..], pet)?;

    Ok(trainer.into())
}
