use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub species: String,
    pub age: u32,
}

impl Pet {
    #[must_use]
    pub const fn new(id: u64, name: String, species: String, age: u32) -> Self {
        Self {
            id,
            name,
            species,
            age,
        }
    }
}
