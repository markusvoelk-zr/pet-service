use serde::{Deserialize, Serialize};
use std::error::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(rename = "Check", skip_serializing_if = "Option::is_none")]
    pub check: Option<ServiceCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCheck {
    #[serde(rename = "HTTP")]
    pub http: String,
    #[serde(rename = "Interval")]
    pub interval: String,
    #[serde(rename = "Timeout")]
    pub timeout: String,
}

pub struct ConsulClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl ConsulClient {
    #[must_use]
    pub fn new(consul_address: &str) -> Self {
        Self {
            base_url: format!("http://{consul_address}"),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Registers a service with Consul.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if Consul returns a non-success status.
    pub fn register_service(
        &self,
        registration: &ServiceRegistration,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/v1/agent/service/register", self.base_url);
        let response = self.client.put(&url).json(registration).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to register service: {}", response.status()).into())
        }
    }

    /// Deregisters a service from Consul.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if Consul returns a non-success status.
    pub fn deregister_service(&self, service_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "{}/v1/agent/service/deregister/{}",
            self.base_url, service_id
        );
        let response = self.client.put(&url).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to deregister service: {}", response.status()).into())
        }
    }
}
