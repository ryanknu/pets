use crate::{
    auth::time,
    error::PetsError,
    types::{Trainer, TrainerInternal},
    Database,
};
/// Creates a trainer
///
pub fn create_trainer<'a>(
    database: &Database,
    username: &String,
    password: &String,
) -> Result<Trainer, &'a str> {
    match create_trainer_inner(database, username, password) {
        Ok(trainer) => Ok(trainer),
        _ => Err("Could not create trainer"),
    }
}

fn create_trainer_inner(
    database: &Database,
    username: &String,
    password: &String,
) -> Result<Trainer, PetsError> {
    if database.trainers.contains_key(username)? {
        return Err(PetsError);
    }

    database.trainers.insert(
        &username[..],
        TrainerInternal {
            name: username.clone(),
            password: password.clone(),
            cash: 100,
            pet_ids: vec![],
            last_seen: time(),
        },
    )?;

    Ok(Trainer {
        name: username.clone(),
        cash: 100,
    })
}
