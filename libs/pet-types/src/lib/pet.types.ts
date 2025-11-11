export interface Pet {
  id: number;
  name: string;
  species: string;
  age: number;
}

export interface CreatePetDto {
  name: string;
  species: string;
  age: number;
}

export interface UpdatePetDto extends CreatePetDto {
  id: number;
}
