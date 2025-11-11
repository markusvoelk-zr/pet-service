import type { Pet } from '@pet-service/pet-types';
import styles from './pet-list.module.scss';

export interface PetListProps {
  pets: Pet[];
  loading: boolean;
  onEdit: (pet: Pet) => void;
  onDelete: (id: number) => void;
}

export function PetList({ pets, loading, onEdit, onDelete }: PetListProps) {
  return (
    <section className={styles.list}>
      <h2>Pet List</h2>
      {loading && <p>Loading...</p>}
      {pets.length === 0 && !loading && <p>No pets found. Add one above!</p>}
      <div className={styles.petGrid}>
        {pets.map((pet) => (
          <div key={pet.id} className={styles.petCard}>
            <h3>{pet.name}</h3>
            <p>
              <strong>Species:</strong> {pet.species}
            </p>
            <p>
              <strong>Age:</strong> {pet.age} years
            </p>
            <p className={styles.petId}>ID: {pet.id}</p>
            <div className={styles.cardButtons}>
              <button onClick={() => onEdit(pet)} disabled={loading}>
                Edit
              </button>
              <button
                onClick={() => onDelete(pet.id)}
                disabled={loading}
                className={styles.deleteBtn}
              >
                Delete
              </button>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}

export default PetList;
