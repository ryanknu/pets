use crate::{
    types::{Trainer, TrainerInternal},
    Database,
};

pub fn adopt_pet<'a>(sled: &Database, username: &str, pet_id: &str) -> Result<Trainer, &'a str> {
    let trainer_result = sled.trainers.get(username);
    match trainer_result {
        Ok(Some(trainer)) => adopt_pet_trainer(sled, trainer, pet_id),
        _ => Err("Trainer DNE"),
    }
}

// TODO: make it cost money. Perhaps add an impl to TrainerInternal that is trainer.pay(&database, cost: i32) -> bool
fn adopt_pet_trainer<'a>(
    sled: &Database,
    trainer: TrainerInternal,
    pet_id: &str,
) -> Result<Trainer, &'a str> {
    let trainer_name = &*(trainer.name.clone());
    let result = sled.adoptable_pets.get(trainer_name);
    if let Ok(Some(pets)) = result {
        let mut found = false;
        let mut new_pets = Vec::new();
        for pet in pets {
            if pet.id.eq(pet_id) {
                sled.pets.insert(&*pet.id, pet.clone());
                found = true;
                // TODO: push on pets vector
                sled.trainers.insert(trainer_name, trainer.clone());
            } else {
                new_pets.insert(new_pets.len(), pet.clone());
            }
        }
        if found {
            sled.adoptable_pets.insert(trainer_name, new_pets);
        }
    }
    Ok(Trainer {
        name: trainer.name,
        cash: trainer.cash,
        pets: vec![], // TODO TODO TODO
    })
}
