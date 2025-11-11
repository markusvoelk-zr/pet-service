import type { Pet, CreatePetDto, UpdatePetDto } from '@pet-service/pet-types';

const API_BASE = '/api';

export class PetApiClient {
  static async getAllPets(): Promise<Pet[]> {
    const response = await fetch(`${API_BASE}/pets`);
    if (!response.ok) {
      throw new Error('Failed to fetch pets');
    }
    return response.json();
  }

  static async getPet(id: number): Promise<Pet> {
    const response = await fetch(`${API_BASE}/pets/${id}`);
    if (!response.ok) {
      throw new Error(`Failed to fetch pet with id ${id}`);
    }
    return response.json();
  }

  static async createPet(pet: CreatePetDto): Promise<Pet> {
    const response = await fetch(`${API_BASE}/pets`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(pet),
    });
    if (!response.ok) {
      throw new Error('Failed to create pet');
    }
    return response.json();
  }

  static async updatePet(pet: UpdatePetDto): Promise<Pet> {
    const response = await fetch(`${API_BASE}/pets/${pet.id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(pet),
    });
    if (!response.ok) {
      throw new Error(`Failed to update pet with id ${pet.id}`);
    }
    return response.json();
  }

  static async deletePet(id: number): Promise<void> {
    const response = await fetch(`${API_BASE}/pets/${id}`, {
      method: 'DELETE',
    });
    if (!response.ok) {
      throw new Error(`Failed to delete pet with id ${id}`);
    }
  }
}
