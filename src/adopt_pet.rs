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

fn adopt_pet_inner<'a>(
    database: &Database,
    username: &String,
    pet_id: &String,
    pet_name: &String,
) -> Result<Trainer, PetsError> {
    let mut trainer = database.trainers.get(username)?.ok_or(PetsError)?;
    let pets = database
        .adoptable_pets
        .get(&trainer.name)?
        .ok_or(PetsError)?;

    let cost = match trainer.pet_ids.len() {
        0 => 0,
        1 => 1_000,
        _ => 10_000,
    };

    let mut adopted_pet = pets
        .iter()
        .find(|pet| pet.id.eq(pet_id))
        .ok_or(PetsError)?
        .clone();
    adopted_pet.name = pet_name.into();

    let mut pets = pets.clone(); // I am unsure if this clone is necessary
    pets.retain(|pet| !pet.id.eq(&adopted_pet.id));

    trainer.pet_ids.insert(trainer.pet_ids.len(), pet_id.into());

    trainer.pay(database, cost)?;
    database
        .pets
        .insert(&*adopted_pet.id, adopted_pet.clone())?;
    database.trainers.insert(&*trainer.name, trainer.clone())?;
    database.adoptable_pets.insert(&*trainer.name, pets)?;

    Ok(Trainer {
        name: trainer.name,
        cash: trainer.cash,
    })
}
