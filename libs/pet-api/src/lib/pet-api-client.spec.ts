import { describe, it, expect, beforeEach, vi } from 'vitest';
import { PetApiClient } from './pet-api-client';
import type { Pet, CreatePetDto, UpdatePetDto } from '@pet-service/pet-types';

describe('PetApiClient', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  describe('getAllPets', () => {
    it('should fetch all pets successfully', async () => {
      const mockPets: Pet[] = [
        { id: 1, name: 'Fluffy', species: 'Cat', age: 3 },
        { id: 2, name: 'Rex', species: 'Dog', age: 5 },
      ];

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockPets,
      });

      const result = await PetApiClient.getAllPets();
      expect(result).toEqual(mockPets);
      expect(global.fetch).toHaveBeenCalledWith('/api/pets');
    });

    it('should throw error when fetch fails', async () => {
      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
      });

      await expect(PetApiClient.getAllPets()).rejects.toThrow(
        'Failed to fetch pets'
      );
    });
  });

  describe('getPet', () => {
    it('should fetch a single pet successfully', async () => {
      const mockPet: Pet = { id: 1, name: 'Fluffy', species: 'Cat', age: 3 };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockPet,
      });

      const result = await PetApiClient.getPet(1);
      expect(result).toEqual(mockPet);
      expect(global.fetch).toHaveBeenCalledWith('/api/pets/1');
    });

    it('should throw error when fetch fails', async () => {
      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
      });

      await expect(PetApiClient.getPet(1)).rejects.toThrow(
        'Failed to fetch pet with id 1'
      );
    });
  });

  describe('createPet', () => {
    it('should create a pet successfully', async () => {
      const createDto: CreatePetDto = { name: 'Buddy', species: 'Dog', age: 4 };
      const mockPet: Pet = { id: 3, ...createDto };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockPet,
      });

      const result = await PetApiClient.createPet(createDto);
      expect(result).toEqual(mockPet);
      expect(global.fetch).toHaveBeenCalledWith('/api/pets', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(createDto),
      });
    });

    it('should throw error when creation fails', async () => {
      const createDto: CreatePetDto = { name: 'Buddy', species: 'Dog', age: 4 };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
      });

      await expect(PetApiClient.createPet(createDto)).rejects.toThrow(
        'Failed to create pet'
      );
    });
  });

  describe('updatePet', () => {
    it('should update a pet successfully', async () => {
      const updateDto: UpdatePetDto = {
        id: 1,
        name: 'Fluffy Updated',
        species: 'Cat',
        age: 4,
      };
      const mockPet: Pet = { ...updateDto };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
        json: async () => mockPet,
      });

      const result = await PetApiClient.updatePet(updateDto);
      expect(result).toEqual(mockPet);
      expect(global.fetch).toHaveBeenCalledWith('/api/pets/1', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updateDto),
      });
    });

    it('should throw error when update fails', async () => {
      const updateDto: UpdatePetDto = {
        id: 1,
        name: 'Fluffy Updated',
        species: 'Cat',
        age: 4,
      };

      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
      });

      await expect(PetApiClient.updatePet(updateDto)).rejects.toThrow(
        'Failed to update pet with id 1'
      );
    });
  });

  describe('deletePet', () => {
    it('should delete a pet successfully', async () => {
      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: true,
      });

      await expect(PetApiClient.deletePet(1)).resolves.toBeUndefined();
      expect(global.fetch).toHaveBeenCalledWith('/api/pets/1', {
        method: 'DELETE',
      });
    });

    it('should throw error when deletion fails', async () => {
      (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        ok: false,
      });

      await expect(PetApiClient.deletePet(1)).rejects.toThrow(
        'Failed to delete pet with id 1'
      );
    });
  });
});