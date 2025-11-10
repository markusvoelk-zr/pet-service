use crate::PetStorage;

#[test]
fn test_add_and_get_pet() {
    let storage = PetStorage::new();
    let pet = storage
        .add_pet("Fluffyy".to_owned(), "Cat".to_owned(), 3)
        .unwrap();
    assert_eq!(pet.id, 1);
    assert_eq!(pet.name, "Fluffyy");

    let retrieved = storage.get_pet(1).unwrap().unwrap();
    assert_eq!(retrieved.name, "Fluffyy");
}

#[test]
fn test_get_all_pets() {
    let storage = PetStorage::new();
    storage
        .add_pet("Fluffy".to_owned(), "Cat".to_owned(), 3)
        .unwrap();
    storage
        .add_pet("Buddy".to_owned(), "Dog".to_owned(), 5)
        .unwrap();

    let all_pets = storage.get_all_pets().unwrap();
    assert_eq!(all_pets.len(), 2);
}

#[test]
fn test_update_pet() {
    let storage = PetStorage::new();
    let pet = storage
        .add_pet("Fluffy".to_owned(), "Cat".to_owned(), 3)
        .unwrap();
    let updated = storage
        .update_pet(pet.id, "Fluffy Jr.".to_owned(), "Cat".to_owned(), 4)
        .unwrap();
    assert!(updated.is_some());
    assert_eq!(updated.unwrap().name, "Fluffy Jr.");
}

#[test]
fn test_delete_pet() {
    let storage = PetStorage::new();
    let pet = storage
        .add_pet("Fluffy".to_owned(), "Cat".to_owned(), 3)
        .unwrap();
    assert!(storage.delete_pet(pet.id).unwrap());
    assert!(storage.get_pet(pet.id).unwrap().is_none());
}
