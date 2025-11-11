import { useState, useEffect } from 'react';
import type { Pet } from '@pet-service/pet-types';
import { PetApiClient } from '@pet-service/pet-api';
import { PetForm } from '@pet-service/pet-form';
import { PetList } from '@pet-service/pet-list';
import styles from './app.module.scss';

export function App() {
  const [pets, setPets] = useState<Pet[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [editingPet, setEditingPet] = useState<Pet | null>(null);

  const fetchPets = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await PetApiClient.getAllPets();
      setPets(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPets();
  }, []);

  const handleSubmit = async (pet: Pet | { name: string; species: string; age: number }) => {
    try {
      setLoading(true);
      setError(null);

      if ('id' in pet) {
        await PetApiClient.updatePet(pet);
      } else {
        await PetApiClient.createPet(pet);
      }

      setEditingPet(null);
      await fetchPets();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  const handleEdit = (pet: Pet) => {
    setEditingPet(pet);
  };

  const handleDelete = async (id: number) => {
    // eslint-disable-next-line no-restricted-globals
    if (!confirm('Are you sure you want to delete this pet?')) return;

    try {
      setLoading(true);
      setError(null);
      await PetApiClient.deletePet(id);
      await fetchPets();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = () => {
    setEditingPet(null);
  };

  return (
    <div className={styles.container}>
      <h1>
        <span role="img" aria-label="paw">
          🐾
        </span>{' '}
        Pet Management
      </h1>

      {error && <div className={styles.error}>{error}</div>}

      <div className={styles.content}>
        <PetForm
          editingPet={editingPet}
          loading={loading}
          onSubmit={handleSubmit}
          onCancel={handleCancel}
        />
        <PetList
          pets={pets}
          loading={loading}
          onEdit={handleEdit}
          onDelete={handleDelete}
        />
      </div>
    </div>
  );
}

export default App;
