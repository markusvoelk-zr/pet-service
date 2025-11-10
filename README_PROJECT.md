# Pet Service - Rust Microservice with Consul

A REST API microservice built with Rust, Actix-Web, and Nx monorepo tooling. The service manages pet records with in-memory storage and automatically registers itself with Consul for service discovery.

## Architecture

This project is organized as an Nx monorepo with the following structure:

### Libraries

1. **models** (`libs/models`)

   - Defines the `Pet` data structure
   - Provides serialization/deserialization support via Serde
   - Contains unit tests for model behavior

2. **storage** (`libs/storage`)

   - Implements in-memory storage for pets using `HashMap` and `Arc<Mutex<>>`
   - Thread-safe operations (add, get, update, delete)
   - Comprehensive test coverage

3. **consul_client** (`libs/consul_client`)
   - Consul service registration client
   - Uses reqwest for HTTP communication with Consul
   - Supports service registration and deregistration

### Application

**web_service** (`apps/web_service`)

- REST API server using Actix-Web
- Exposes `/pets` endpoint with full CRUD operations
- Health check endpoint at `/health`
- Automatically registers with Consul on startup

## API Endpoints

### Health Check

- **GET** `/health` - Returns service health status

### Pet Management

- **GET** `/pets` - Get all pets
- **GET** `/pets/{id}` - Get a specific pet by ID
- **POST** `/pets` - Create a new pet
  ```json
  {
    "name": "Fluffy",
    "species": "Cat",
    "age": 3
  }
  ```
- **PUT** `/pets/{id}` - Update an existing pet
  ```json
  {
    "name": "Fluffy Jr.",
    "species": "Cat",
    "age": 4
  }
  ```
- **DELETE** `/pets/{id}` - Delete a pet

## Prerequisites

- Rust (via rustup): https://rustup.rs/
- Node.js LTS
- Consul server running on `127.0.0.1:8500` (optional, service will warn if unavailable)

## Installation

```bash
# Install dependencies
npm install
```

## Development Commands

### Build

```bash
# Build the web service
npx nx build web_service

# Build all projects
npx nx run-many -t build
```

### Test

```bash
# Test a specific project
npx nx test models
npx nx test storage
npx nx test consul_client
npx nx test web_service

# Test all projects
npx nx run-many -t test
```

### Lint

```bash
# Lint a specific project
npx nx lint web_service

# Lint all projects
npx nx run-many -t lint
```

### Run

```bash
# Run the web service
npx nx run web_service
```

## Running with Consul

1. Start Consul (if available):

```bash
consul agent -dev
```

2. Start the pet service:

```bash
npx nx run web_service
```

The service will:

- Start on `http://127.0.0.1:8080`
- Attempt to register with Consul at `127.0.0.1:8500`
- If Consul is unavailable, it will log a warning but continue running

3. Verify registration in Consul:

```bash
curl http://127.0.0.1:8500/v1/agent/services
```

## Testing the API

### Using Bruno API Client (Recommended)

A complete Bruno API collection is included in the `.bruno` folder with:

- 12 pre-configured requests covering all endpoints
- 50+ automated test assertions
- Request chaining with variables
- Error handling tests
- Consul integration verification

**Quick Start:**

1. Install Bruno from https://www.usebruno.com/
2. Open the `.bruno` folder as a collection
3. Select "Local" environment
4. Run requests individually or use "Run Collection"

See [.bruno/README.md](.bruno/README.md) for detailed documentation.

### Using curl

```bash
# Health check
curl http://127.0.0.1:8080/health

# Get all pets
curl http://127.0.0.1:8080/pets

# Create a pet
curl -X POST http://127.0.0.1:8080/pets \
  -H "Content-Type: application/json" \
  -d '{"name":"Buddy","species":"Dog","age":5}'

# Get a specific pet
curl http://127.0.0.1:8080/pets/1

# Update a pet
curl -X PUT http://127.0.0.1:8080/pets/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Buddy Sr.","species":"Dog","age":6}'

# Delete a pet
curl -X DELETE http://127.0.0.1:8080/pets/1
```

### Using Bruno CLI

```bash
# Install Bruno CLI
npm install -g @usebruno/cli

# Run all tests
bru run .bruno --env Local

# Run specific folder
bru run .bruno/Pets --env Local
```

## Project Structure

```
pet-service/
├── apps/
│   └── web_service/          # Main application
│       ├── src/
│       │   └── main.rs       # HTTP server and routes
│       └── Cargo.toml
├── libs/
│   ├── models/               # Pet data model
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── storage/              # In-memory storage
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── consul_client/        # Consul integration
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
├── Cargo.toml                # Workspace configuration
├── nx.json                   # Nx configuration
└── package.json              # Node dependencies
```

## Features

✅ REST API with Actix-Web
✅ In-memory storage with thread-safe operations
✅ Consul service registration with health checks
✅ Comprehensive unit tests
✅ Lint checks with Clippy
✅ Nx monorepo structure
✅ Multiple libraries (models, storage, consul_client)
✅ Full CRUD operations for pets
✅ Error handling with proper HTTP status codes

## Technology Stack

- **Language**: Rust 2021 Edition
- **Web Framework**: Actix-Web 4
- **Serialization**: Serde & Serde JSON
- **HTTP Client**: reqwest (blocking)
- **Build Tool**: Nx with @monodon/rust
- **Service Discovery**: Consul

## Notes

- The service uses in-memory storage, so data is not persisted across restarts
- Pet IDs are auto-incremented starting from 1
- The service will attempt to register with Consul but will continue running even if Consul is unavailable
- All endpoints return JSON responses
- Proper HTTP status codes are used (200, 201, 204, 404, 500)
