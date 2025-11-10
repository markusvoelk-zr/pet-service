use crate::Pet;

#[test]
fn test_pet_creation() {
    let pet = Pet::new(1, "Fluffy".to_owned(), "Cat".to_owned(), 3);
    assert_eq!(pet.id, 1);
    assert_eq!(pet.name, "Fluffy");
    assert_eq!(pet.species, "Cat");
    assert_eq!(pet.age, 3);
}

#[test]
fn test_pet_clone() {
    let pet1 = Pet::new(1, "Buddy".to_owned(), "Dog".to_owned(), 5);
    let pet2 = pet1.clone();
    assert_eq!(pet1, pet2);
}
