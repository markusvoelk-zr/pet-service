import { describe, it, expect } from 'vitest';
import type { Pet, CreatePetDto, UpdatePetDto } from './pet.types';

describe('Pet Types', () => {
  it('should define Pet interface correctly', () => {
    const pet: Pet = {
      id: 1,
      name: 'Fluffy',
      species: 'Cat',
      age: 3,
    };

    expect(pet.id).toBe(1);
    expect(pet.name).toBe('Fluffy');
    expect(pet.species).toBe('Cat');
    expect(pet.age).toBe(3);
  });

  it('should define CreatePetDto interface correctly', () => {
    const createPetDto: CreatePetDto = {
      name: 'Rex',
      species: 'Dog',
      age: 5,
    };

    expect(createPetDto.name).toBe('Rex');
    expect(createPetDto.species).toBe('Dog');
    expect(createPetDto.age).toBe(5);
  });

  it('should define UpdatePetDto interface correctly', () => {
    const updatePetDto: UpdatePetDto = {
      id: 2,
      name: 'Buddy',
      species: 'Dog',
      age: 4,
    };

    expect(updatePetDto.id).toBe(2);
    expect(updatePetDto.name).toBe('Buddy');
    expect(updatePetDto.species).toBe('Dog');
    expect(updatePetDto.age).toBe(4);
  });
});