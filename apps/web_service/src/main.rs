use actix_web::{App, HttpServer, web};
use consul_client::{ConsulClient, ServiceCheck, ServiceRegistration};
use std::sync::Arc;
use storage::PetStorage;

use crate::routes::{
    AppState, create_pet, delete_pet, get_all_pets, get_pet, health_check,
};

mod routes;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8081;
const CONSUL_ADDRESS: &str = "127.0.0.1:8500";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize storage
    let storage = PetStorage::new();
    let app_state = Arc::new(AppState { storage });

    tokio::task::spawn_blocking(move || {
        let consul_client = ConsulClient::new(CONSUL_ADDRESS);
        let registration = ServiceRegistration {
            id: "pet-service-1".to_owned(),
            name: "pet-service".to_owned(),
            tags: vec!["pets".to_owned(), "rest".to_owned(), "api".to_owned()],
            address: HOST.to_owned(),
            port: PORT,
            check: Some(ServiceCheck {
                http: format!("http://{HOST}:{PORT}/health"),
                interval: "10s".to_owned(),
                timeout: "5s".to_owned(),
            }),
        };

        match consul_client.register_service(&registration) {
            Ok(()) => println!("Successfully registered service with Consul"),
            Err(e) => eprintln!("Warning: Failed to register with Consul: {e}"),
        }
    });

    println!("Starting Pet Service on {HOST}:{PORT}");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_state)))
            .route("/health", web::get().to(health_check))
            .route("/pets", web::get().to(get_all_pets))
            .route("/pets/{id}", web::get().to(get_pet))
            .route("/pets", web::post().to(create_pet))
            .route("/pets/{id}", web::delete().to(delete_pet))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
