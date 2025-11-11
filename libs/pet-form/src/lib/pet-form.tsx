import { useState, useEffect } from 'react';
import type { Pet, CreatePetDto } from '@pet-service/pet-types';
import styles from './pet-form.module.scss';

export interface PetFormProps {
  editingPet: Pet | null;
  loading: boolean;
  onSubmit: (pet: CreatePetDto | Pet) => Promise<void>;
  onCancel: () => void;
}

export function PetForm({
  editingPet,
  loading,
  onSubmit,
  onCancel,
}: PetFormProps) {
  const [formData, setFormData] = useState({
    name: '',
    species: '',
    age: '',
  });

  useEffect(() => {
    if (editingPet) {
      setFormData({
        name: editingPet.name,
        species: editingPet.species,
        age: editingPet.age.toString(),
      });
    } else {
      setFormData({ name: '', species: '', age: '' });
    }
  }, [editingPet]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const pet: CreatePetDto = {
      name: formData.name,
      species: formData.species,
      age: parseInt(formData.age),
    };

    if (editingPet) {
      await onSubmit({ ...pet, id: editingPet.id });
    } else {
      await onSubmit(pet);
    }

    setFormData({ name: '', species: '', age: '' });
  };

  return (
    <section className={styles.form}>
      <h2>{editingPet ? 'Edit Pet' : 'Add New Pet'}</h2>
      <form onSubmit={handleSubmit}>
        <div className={styles.formGroup}>
          <label htmlFor="name">Name:</label>
          <input
            type="text"
            id="name"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            required
            disabled={loading}
          />
        </div>

        <div className={styles.formGroup}>
          <label htmlFor="species">Species:</label>
          <input
            type="text"
            id="species"
            value={formData.species}
            onChange={(e) =>
              setFormData({ ...formData, species: e.target.value })
            }
            required
            disabled={loading}
          />
        </div>

        <div className={styles.formGroup}>
          <label htmlFor="age">Age:</label>
          <input
            type="number"
            id="age"
            value={formData.age}
            onChange={(e) => setFormData({ ...formData, age: e.target.value })}
            required
            min="0"
            disabled={loading}
          />
        </div>

        <div className={styles.buttons}>
          <button type="submit" disabled={loading}>
            {editingPet ? 'Update' : 'Create'} Pet
          </button>
          {editingPet && (
            <button type="button" onClick={onCancel} disabled={loading}>
              Cancel
            </button>
          )}
        </div>
      </form>
    </section>
  );
}

export default PetForm;
