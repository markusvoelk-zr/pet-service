use models::Pet;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct PetStorage {
    pets: Arc<Mutex<HashMap<u64, Pet>>>,
    next_id: Arc<Mutex<u64>>,
}

impl PetStorage {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pets: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Adds a new pet to the storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the mutex lock is poisoned.
    pub fn add_pet(&self, name: String, species: String, age: u32) -> Result<Pet, String> {
        let mut id_lock = self.next_id.lock().map_err(|e| e.to_string())?;
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);

        let pet = Pet::new(id, name, species, age);
        self.pets
            .lock()
            .map_err(|e| e.to_string())?
            .insert(id, pet.clone());
        Ok(pet)
    }

    /// Retrieves a pet by its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the mutex lock is poisoned.
    pub fn get_pet(&self, id: u64) -> Result<Option<Pet>, String> {
        let pets = self.pets.lock().map_err(|e| e.to_string())?;
        Ok(pets.get(&id).cloned())
    }

    /// Retrieves all pets from the storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the mutex lock is poisoned.
    pub fn get_all_pets(&self) -> Result<Vec<Pet>, String> {
        let pets = self.pets.lock().map_err(|e| e.to_string())?;
        Ok(pets.values().cloned().collect())
    }

    /// Updates an existing pet in the storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the mutex lock is poisoned.
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

    /// Deletes a pet from the storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the mutex lock is poisoned.
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
