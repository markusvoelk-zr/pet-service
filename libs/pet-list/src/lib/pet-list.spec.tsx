import { render } from '@testing-library/react';
import { vi } from 'vitest';
import PetList from './pet-list';

describe('PetList', () => {
  it('should render successfully with empty list', () => {
    const mockOnEdit = vi.fn();
    const mockOnDelete = vi.fn();

    const { getByText } = render(
      <PetList
        pets={[]}
        loading={false}
        onEdit={mockOnEdit}
        onDelete={mockOnDelete}
      />
    );
    expect(getByText('Pet List')).toBeTruthy();
    expect(getByText('No pets found. Add one above!')).toBeTruthy();
  });

  it('should render pets when provided', () => {
    const mockPets = [
      { id: 1, name: 'Fluffy', species: 'Cat', age: 3 },
      { id: 2, name: 'Rex', species: 'Dog', age: 5 },
    ];
    const mockOnEdit = vi.fn();
    const mockOnDelete = vi.fn();

    const { getByText } = render(
      <PetList
        pets={mockPets}
        loading={false}
        onEdit={mockOnEdit}
        onDelete={mockOnDelete}
      />
    );
    expect(getByText('Fluffy')).toBeTruthy();
    expect(getByText('Rex')).toBeTruthy();
  });

  it('should show loading state', () => {
    const mockOnEdit = vi.fn();
    const mockOnDelete = vi.fn();

    const { getByText } = render(
      <PetList
        pets={[]}
        loading={true}
        onEdit={mockOnEdit}
        onDelete={mockOnDelete}
      />
    );
    expect(getByText('Loading...')).toBeTruthy();
  });
});
