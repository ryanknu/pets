use crate::{
    auth::time,
    types::{Trainer, TrainerInternal},
    Database,
};
/// Creates a trainer
///
pub fn create_trainer<'a>(
    sled: &Database,
    username: &str,
    password: &str,
) -> Result<Trainer, &'a str> {
    let result = sled.trainers.get(username);
    if let Ok(Some(_)) = result {
        return Err("Trainer already exists");
    }

    let new_trainer = TrainerInternal {
        name: username.into(),
        password: password.into(),
        cash: 100,
        pet_ids: vec![],
        last_seen: time(),
    };

    let result = sled.trainers.insert(username, new_trainer.clone());
    if let Ok(None) = result {
        return Ok(Trainer {
            name: new_trainer.name,
            cash: new_trainer.cash,
        });
    }

    Err("Could not save trainer")
}
