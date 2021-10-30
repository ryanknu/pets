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
    // RK: REPORT: I noticed that neither Ok(Some()) nor Err() match result. This probably means it's Ok(None),
    //             which probably means that I can just return new_trainer.
    let result = sled.trainers.insert(username, new_trainer);
    if let Ok(Some(new_trainer)) = result {
        return Ok(Trainer {
            name: new_trainer.name,
            cash: new_trainer.cash,
        });
    }
    if let Err(msg) = result {
        print!("{:?}", msg);
    }

    Err("Could not save trainer")
}
