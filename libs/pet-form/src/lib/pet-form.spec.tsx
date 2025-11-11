import { render } from '@testing-library/react';

import PetForm from './pet-form';

describe('PetForm', () => {
  it('should render successfully', () => {
    const { baseElement } = render(<PetForm />);
    expect(baseElement).toBeTruthy();
  });
});
