use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use storage::PetStorage;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePetRequest {
    name: String,
    species: String,
    age: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePetRequest {
    name: String,
    species: String,
    age: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

pub struct AppState {
    pub(crate) storage: PetStorage,
}

// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

// GET /pets - Get all pets
pub async fn get_all_pets(data: web::Data<Arc<AppState>>) -> impl Responder {
    match data.storage.get_all_pets() {
        Ok(pets) => HttpResponse::Ok().json(pets),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// GET /pets/{id} - Get a specific pet by ID
pub async fn get_pet(data: web::Data<Arc<AppState>>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    match data.storage.get_pet(id) {
        Ok(Some(pet)) => HttpResponse::Ok().json(pet),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Pet with ID {id} not found"),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// POST /pets - Create a new pet
pub async fn create_pet(
    data: web::Data<Arc<AppState>>,
    req: web::Json<CreatePetRequest>,
) -> impl Responder {
    match data
        .storage
        .add_pet(req.name.clone(), req.species.clone(), req.age)
    {
        Ok(pet) => HttpResponse::Created().json(pet),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// DELETE /pets/{id} - Delete a pet
pub async fn delete_pet(data: web::Data<Arc<AppState>>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    match data.storage.delete_pet(id) {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Pet with ID {id} not found"),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}
