use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use consul_client::{ConsulClient, ServiceCheck, ServiceRegistration};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use storage::PetStorage;

#[derive(Debug, Serialize, Deserialize)]
struct CreatePetRequest {
    name: String,
    species: String,
    age: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePetRequest {
    name: String,
    species: String,
    age: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

struct AppState {
    storage: PetStorage,
}

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

// GET /pets - Get all pets
async fn get_all_pets(data: web::Data<Arc<AppState>>) -> impl Responder {
    match data.storage.get_all_pets() {
        Ok(pets) => HttpResponse::Ok().json(pets),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e,
        }),
    }
}

// GET /pets/{id} - Get a specific pet by ID
async fn get_pet(data: web::Data<Arc<AppState>>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    match data.storage.get_pet(id) {
        Ok(Some(pet)) => HttpResponse::Ok().json(pet),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Pet with ID {id} not found"),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e,
        }),
    }
}

// POST /pets - Create a new pet
async fn create_pet(
    data: web::Data<Arc<AppState>>,
    req: web::Json<CreatePetRequest>,
) -> impl Responder {
    match data
        .storage
        .add_pet(req.name.clone(), req.species.clone(), req.age)
    {
        Ok(pet) => HttpResponse::Created().json(pet),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e,
        }),
    }
}

// PUT /pets/{id} - Update a pet
async fn update_pet(
    data: web::Data<Arc<AppState>>,
    path: web::Path<u64>,
    req: web::Json<UpdatePetRequest>,
) -> impl Responder {
    let id = path.into_inner();
    match data
        .storage
        .update_pet(id, req.name.clone(), req.species.clone(), req.age)
    {
        Ok(Some(pet)) => HttpResponse::Ok().json(pet),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Pet with ID {id} not found"),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e,
        }),
    }
}

// DELETE /pets/{id} - Delete a pet
async fn delete_pet(data: web::Data<Arc<AppState>>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    match data.storage.delete_pet(id) {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("Pet with ID {id} not found"),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8081u16;
    let consul_address = "127.0.0.1:8500";

    // Initialize storage
    let storage = PetStorage::new();
    let app_state = Arc::new(AppState { storage });

    // Register service with Consul in a blocking task
    let consul_address_clone = consul_address.to_owned();
    let host_clone = host.to_owned();
    let port_clone = port;

    tokio::task::spawn_blocking(move || {
        let consul_client = ConsulClient::new(&consul_address_clone);
        let registration = ServiceRegistration {
            id: "pet-service-1".to_owned(),
            name: "pet-service".to_owned(),
            tags: vec!["pets".to_owned(), "rest".to_owned(), "api".to_owned()],
            address: host_clone.clone(),
            port: port_clone,
            check: Some(ServiceCheck {
                http: format!("http://{host_clone}:{port_clone}/health"),
                interval: "10s".to_owned(),
                timeout: "5s".to_owned(),
            }),
        };

        match consul_client.register_service(&registration) {
            Ok(()) => println!("Successfully registered service with Consul"),
            Err(e) => eprintln!("Warning: Failed to register with Consul: {e}"),
        }
    });

    println!("Starting Pet Service on {host}:{port}");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_state)))
            .route("/health", web::get().to(health_check))
            .route("/pets", web::get().to(get_all_pets))
            .route("/pets/{id}", web::get().to(get_pet))
            .route("/pets", web::post().to(create_pet))
            .route("/pets/{id}", web::put().to(update_pet))
            .route("/pets/{id}", web::delete().to(delete_pet))
    })
    .bind((host, port))?
    .run()
    .await
}
