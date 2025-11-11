import { render, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import App from './app';

// Mock the PetApiClient
vi.mock('@pet-service/pet-api', () => ({
  PetApiClient: {
    getAllPets: vi.fn().mockResolvedValue([]),
    createPet: vi.fn(),
    updatePet: vi.fn(),
    deletePet: vi.fn(),
  },
}));

describe('App', () => {
  it('should render successfully', async () => {
    const { baseElement } = render(<App />);
    await waitFor(() => {
      expect(baseElement).toBeTruthy();
    });
  });

  it('should have Pet Management as the title', async () => {
    const { getByText } = render(<App />);
    await waitFor(() => {
      expect(getByText('Pet Management')).toBeTruthy();
    });
  });

  it('should render the form and list sections', async () => {
    const { getByText } = render(<App />);
    await waitFor(() => {
      expect(getByText('Add New Pet')).toBeTruthy();
      expect(getByText('Pet List')).toBeTruthy();
    });
  });
});
