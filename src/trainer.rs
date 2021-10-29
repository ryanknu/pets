use crate::{types::Trainer, Database};
/// Gets the current trainer
///
pub fn trainer<'a>(sled: &Database, username: &str) -> Result<Trainer, &'a str> {
    let result = sled.trainers.get(username);
    if let Ok(Some(new_trainer)) = result {
        return Ok(new_trainer.into());
    }

    Err("Trainer not found")
}
