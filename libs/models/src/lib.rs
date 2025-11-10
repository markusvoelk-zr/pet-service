use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub species: String,
    pub age: u32,
}

impl Pet {
    pub fn new(id: u64, name: String, species: String, age: u32) -> Self {
        Pet {
            id,
            name,
            species,
            age,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_creation() {
        let pet = Pet::new(1, "Fluffy".to_string(), "Cat".to_string(), 3);
        assert_eq!(pet.id, 1);
        assert_eq!(pet.name, "Fluffy");
        assert_eq!(pet.species, "Cat");
        assert_eq!(pet.age, 3);
    }

    #[test]
    fn test_pet_clone() {
        let pet1 = Pet::new(1, "Buddy".to_string(), "Dog".to_string(), 5);
        let pet2 = pet1.clone();
        assert_eq!(pet1, pet2);
    }
}
