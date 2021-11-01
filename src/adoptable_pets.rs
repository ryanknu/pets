use crate::{
    types::{Pet, PetInternal, Species},
    Database,
};

/// # Adoptable pets
/// Retrieves adoptable pets for a user from Sled. If they do not exist, then
/// we will generate three pets and put them in Sled.
///
/// If Sled does not work
/// correctly, it will just return three new pets, to simplify code pathways
/// that call this method.
pub fn adoptable_pets(sled: &Database, username: &str) -> Vec<Pet> {
    match sled.adoptable_pets.get(username) {
        Ok(Some(adoptable_pets)) => adoptable_pets.iter().map(|p| p.clone().into()).collect(),
        _ => create_pets(sled, username),
    }
}

fn create_pets(sled: &Database, username: &str) -> Vec<Pet> {
    let random_pets = three_random_pets(username);
    let result = sled.adoptable_pets.insert(username, random_pets.clone());
    let random_pets = random_pets.iter().map(|p| p.clone().into()).collect();
    if let Err(_) = result {
        return random_pets;
    }
    random_pets
}

fn random_pet(username: &str) -> PetInternal {
    PetInternal {
        id: format!("{}", fastrand::i32(0..2000000000)),
        trainer_name: Some(username.into()),
        name: "Unnamed Pet".into(),
        species: random_species(),
        age: 1.0,
        last_fed: 0,
        art_seed: fastrand::i32(0..2000000000),
    }
}

fn three_random_pets(username: &str) -> Vec<PetInternal> {
    vec![
        random_pet(username),
        random_pet(username),
        random_pet(username),
    ]
}

fn random_species() -> Species {
    let v = fastrand::i32(1..4);
    match v {
        1 => Species::Flarf,
        2 => Species::Glabor,
        _ => Species::Tiqqa,
    }
}
