use crate::{types::Trainer, Database};

pub fn pet_to_trainer(database: &Database, trainer_name: String) -> Option<Trainer> {
    if let Ok(Some(trainer)) = database.trainers.get(trainer_name) {
        return Some(trainer.into());
    }
    None
}
