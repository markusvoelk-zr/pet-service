use crate::{ConsulClient, ServiceCheck, ServiceRegistration};

#[test]
fn test_service_registration_creation() {
    let registration = ServiceRegistration {
        id: "pet-service-1".to_owned(),
        name: "pet-service".to_owned(),
        tags: vec!["pets".to_owned(), "rest".to_owned()],
        address: "127.0.0.1".to_owned(),
        port: 8080,
        check: Some(ServiceCheck {
            http: "http://127.0.0.1:8080/health".to_owned(),
            interval: "10s".to_owned(),
            timeout: "5s".to_owned(),
        }),
    };

    assert_eq!(registration.id, "pet-service-1");
    assert_eq!(registration.name, "pet-service");
    assert_eq!(registration.port, 8080);
}

#[test]
fn test_consul_client_creation() {
    let client = ConsulClient::new("127.0.0.1:8500");
    assert_eq!(client.base_url, "http://127.0.0.1:8500");
}
