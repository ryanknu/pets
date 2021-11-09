use crate::{error::PetsError, types::Trainer, Database};

pub fn adopt_pet<'a>(
    sled: &Database,
    username: &String,
    pet_id: &String,
    pet_name: &String,
) -> Result<Trainer, &'a str> {
    match adopt_pet_inner(sled, username, pet_id, pet_name) {
        Ok(trainer) => Ok(trainer),
        _ => Err("Trainer DNE"), // TODO: this error message no longer covers.
    }
}

// TODO: Make it cost money. Perhaps add an impl to TrainerInternal that is trainer.pay(&database, cost: i32) -> bool?
// TODO: Method needs a clean up. new_pets should be a filter call, etc.
// TODO: Send back actual error messages
fn adopt_pet_inner<'a>(
    sled: &Database,
    username: &String,
    pet_id: &String,
    pet_name: &String,
) -> Result<Trainer, PetsError> {
    let trainer = sled.trainers.get(username)?.ok_or(PetsError)?;
    let pets = sled.adoptable_pets.get(&trainer.name)?.ok_or(PetsError)?;

    let mut found = false;
    let mut new_pets = Vec::new();
    for pet in pets {
        if pet.id.eq(pet_id) {
            let mut adopted_pet = pet.clone();
            adopted_pet.name = pet_name.into();
            sled.pets.insert(&*pet.id, adopted_pet)?;
            found = true;

            let mut new_trainer = trainer.clone();
            new_trainer
                .pet_ids
                .insert(new_trainer.pet_ids.len(), pet.id.into());
            sled.trainers.insert(&*trainer.name, new_trainer)?;
        } else {
            new_pets.insert(new_pets.len(), pet.clone());
        }
    }
    if found {
        sled.adoptable_pets.insert(&*trainer.name, new_pets)?;
    }

    Ok(Trainer {
        name: trainer.name,
        cash: trainer.cash,
    })
}
