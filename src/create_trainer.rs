use crate::{
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
    let new_trainer = TrainerInternal {
        name: username.into(),
        password: password.into(),
        cash: 100,
        pet_ids: vec![],
    };
    let result = sled.trainers.insert(username, new_trainer);
    if let Ok(Some(new_trainer)) = result {
        return Ok(Trainer {
            name: new_trainer.name,
            cash: new_trainer.cash,
            pets: vec![],
        });
    }

    Err("Could not save trainer")
}
