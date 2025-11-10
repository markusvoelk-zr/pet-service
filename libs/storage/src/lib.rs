use models::Pet;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PetStorage {
    pets: Arc<Mutex<HashMap<u64, Pet>>>,
    next_id: Arc<Mutex<u64>>,
}

impl PetStorage {
    pub fn new() -> Self {
        PetStorage {
            pets: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn add_pet(&self, name: String, species: String, age: u32) -> Result<Pet, String> {
        let mut id_lock = self.next_id.lock().map_err(|e| e.to_string())?;
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);

        let pet = Pet::new(id, name, species, age);
        let mut pets = self.pets.lock().map_err(|e| e.to_string())?;
        pets.insert(id, pet.clone());
        Ok(pet)
    }

    pub fn get_pet(&self, id: u64) -> Result<Option<Pet>, String> {
        let pets = self.pets.lock().map_err(|e| e.to_string())?;
        Ok(pets.get(&id).cloned())
    }

    pub fn get_all_pets(&self) -> Result<Vec<Pet>, String> {
        let pets = self.pets.lock().map_err(|e| e.to_string())?;
        Ok(pets.values().cloned().collect())
    }

    pub fn update_pet(
        &self,
        id: u64,
        name: String,
        species: String,
        age: u32,
    ) -> Result<Option<Pet>, String> {
        let mut pets = self.pets.lock().map_err(|e| e.to_string())?;
        if let std::collections::hash_map::Entry::Occupied(mut e) = pets.entry(id) {
            let pet = Pet::new(id, name, species, age);
            e.insert(pet.clone());
            Ok(Some(pet))
        } else {
            Ok(None)
        }
    }

    pub fn delete_pet(&self, id: u64) -> Result<bool, String> {
        let mut pets = self.pets.lock().map_err(|e| e.to_string())?;
        Ok(pets.remove(&id).is_some())
    }
}

impl Default for PetStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_pet() {
        let storage = PetStorage::new();
        let pet = storage
            .add_pet("Fluffy".to_string(), "Cat".to_string(), 3)
            .unwrap();
        assert_eq!(pet.id, 1);
        assert_eq!(pet.name, "Fluffy");

        let retrieved = storage.get_pet(1).unwrap().unwrap();
        assert_eq!(retrieved.name, "Fluffy");
    }

    #[test]
    fn test_get_all_pets() {
        let storage = PetStorage::new();
        storage
            .add_pet("Fluffy".to_string(), "Cat".to_string(), 3)
            .unwrap();
        storage
            .add_pet("Buddy".to_string(), "Dog".to_string(), 5)
            .unwrap();

        let all_pets = storage.get_all_pets().unwrap();
        assert_eq!(all_pets.len(), 2);
    }

    #[test]
    fn test_update_pet() {
        let storage = PetStorage::new();
        let pet = storage
            .add_pet("Fluffy".to_string(), "Cat".to_string(), 3)
            .unwrap();
        let updated = storage
            .update_pet(pet.id, "Fluffy Jr.".to_string(), "Cat".to_string(), 4)
            .unwrap();
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().name, "Fluffy Jr.");
    }

    #[test]
    fn test_delete_pet() {
        let storage = PetStorage::new();
        let pet = storage
            .add_pet("Fluffy".to_string(), "Cat".to_string(), 3)
            .unwrap();
        assert!(storage.delete_pet(pet.id).unwrap());
        assert!(storage.get_pet(pet.id).unwrap().is_none());
    }
}
