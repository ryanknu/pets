use crate::{
    types::{PetInternal, TrainerInternal, UpdatePet, UpdateResult},
    AuthedContext,
};

/// # Update
///
/// Performs a game tick.
pub fn update<'a>(
    context: &AuthedContext,
    username: String,
    now: u64,
) -> Result<UpdateResult, &'a str> {
    match context.database.trainers.get(username) {
        Ok(Some(trainer)) => update_trainer(context, trainer, now),
        _ => Err("Trainer not found"),
    }
}

fn update_trainer<'a>(
    context: &AuthedContext,
    trainer: TrainerInternal,
    now: u64,
) -> Result<UpdateResult, &'a str> {
    let mut pet_updates = Vec::new();
    let mut new_trainer = trainer.clone();
    let mut total_cash = 0;
    let duration = now - trainer.last_seen;

    // Update pets
    for pet_id in trainer.pet_ids {
        if let Ok(result) = update_pet_id(context, pet_id, duration) {
            total_cash += result.cash_earned;
            pet_updates.insert(pet_updates.len(), result);
        } else {
            return Err("Error updating a pet");
        }
    }

    // Update trainer
    new_trainer.cash += total_cash;
    match context
        .database
        .trainers
        .insert(&*trainer.name, new_trainer)
    {
        Ok(Some(_)) => Ok(UpdateResult {
            duration: (duration as i32),
            pets: pet_updates,
        }),
        _ => Err("Error saving trainer"),
    }
}

fn update_pet_id<'a>(
    context: &AuthedContext,
    pet_id: String,
    duration: u64,
) -> Result<UpdatePet, &'a str> {
    match context.database.pets.get(pet_id) {
        Ok(Some(pet)) => update_pet(context, pet, duration),
        _ => Err("Could not find pet"),
    }
}

fn update_pet<'a>(
    context: &AuthedContext,
    pet: PetInternal,
    duration: u64,
) -> Result<UpdatePet, &'a str> {
    let max_hungry = 3600 * (pet.age as u64);
    let hungry = if duration > max_hungry {
        max_hungry
    } else {
        duration
    };
    let cash_earned = 25.0 * ((hungry as f64) / 3600.0);
    let cash_earned = cash_earned as i32;

    let mut new_pet = pet.clone();
    new_pet.age += (hungry as f64) / 86400.0;
    match context.database.pets.insert(&*pet.id, new_pet.clone()) {
        Ok(Some(pet)) => Ok(UpdatePet {
            pet_id: pet.id,
            from_age: pet.age,
            to_age: new_pet.age,
            cash_earned: cash_earned,
        }),
        _ => Err("Error updating pet"),
    }
}
