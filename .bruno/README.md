# Bruno API Collection - Pet Service

This Bruno collection provides a complete set of API requests to test the Pet Service REST API.

## Overview

The collection includes:

- **Health Check**: Verify service is running
- **Pet CRUD Operations**: Complete Create, Read, Update, Delete tests
- **Error Handling Tests**: Verify 404 responses for nonexistent resources
- **Consul Integration**: Verify service registration and health checks

## Prerequisites

1. **Install Bruno**: Download from https://www.usebruno.com/
2. **Start Pet Service**: Run `npx nx run web_service` from the project root
3. **Optional - Consul**: Start Consul server for integration tests: `consul agent -dev`

## Collection Structure

```
.bruno/
├── bruno.json                          # Collection configuration
├── environments/
│   └── Local.bru                       # Local environment variables
├── Health Check.bru                    # Service health check
├── Pets/
│   ├── Get All Pets.bru               # List all pets
│   ├── Create Pet.bru                 # Create a pet (saves ID to variable)
│   ├── Create Multiple Pets.bru       # Create additional pets
│   ├── Get Pet by ID.bru              # Get specific pet by ID
│   ├── Get Nonexistent Pet.bru        # Test 404 error
│   ├── Update Pet.bru                 # Update a pet
│   ├── Update Nonexistent Pet.bru     # Test 404 on update
│   ├── Delete Pet.bru                 # Delete a pet
│   └── Delete Nonexistent Pet.bru     # Test 404 on delete
└── Consul/
    ├── Verify Consul Registration.bru  # Check service registration
    └── Check Pet Service Health.bru    # Check Consul health status
```

## Quick Start

### 1. Open Collection in Bruno

1. Open Bruno application
2. Click "Open Collection"
3. Navigate to `c:\dev\rust-playground\pet-service\.bruno`
4. Select the folder

### 2. Select Environment

1. In Bruno, click on "No Environment" dropdown
2. Select "Local"
3. Verify variables:
   - `baseUrl`: http://127.0.0.1:8080
   - `consulUrl`: http://127.0.0.1:8500

### 3. Run Requests

#### Manual Testing

Click on any request and press "Send" or Ctrl+Enter to execute.

#### Automated Testing

1. Right-click on the collection root
2. Select "Run Collection"
3. All requests will execute in sequence
4. View test results in the output

## Request Details

### Health Check

- **Method**: GET
- **URL**: `/health`
- **Purpose**: Verify service is running
- **Expected**: 200 with `{"status":"healthy"}`

### Get All Pets

- **Method**: GET
- **URL**: `/pets`
- **Purpose**: List all pets
- **Expected**: 200 with array of pets

### Create Pet

- **Method**: POST
- **URL**: `/pets`
- **Body**:
  ```json
  {
    "name": "Fluffy",
    "species": "Cat",
    "age": 3
  }
  ```
- **Expected**: 201 with created pet (includes auto-generated ID)
- **Side Effect**: Saves `petId` variable for subsequent requests

### Get Pet by ID

- **Method**: GET
- **URL**: `/pets/{id}`
- **Purpose**: Retrieve specific pet
- **Expected**: 200 with pet details
- **Note**: Uses `{{petId}}` variable from Create Pet

### Update Pet

- **Method**: PUT
- **URL**: `/pets/{id}`
- **Body**:
  ```json
  {
    "name": "Fluffy Jr.",
    "species": "Cat",
    "age": 4
  }
  ```
- **Expected**: 200 with updated pet

### Delete Pet

- **Method**: DELETE
- **URL**: `/pets/{id}`
- **Expected**: 204 No Content

### Error Handling

The collection includes tests for:

- Getting nonexistent pet (404)
- Updating nonexistent pet (404)
- Deleting nonexistent pet (404)

### Consul Integration

- **Verify Registration**: Checks if service appears in Consul
- **Health Check**: Verifies Consul health monitoring

## Variables

### Collection Variables

- `petId`: Auto-populated by "Create Pet" request

### Environment Variables

- `baseUrl`: Pet service base URL (default: http://127.0.0.1:8080)
- `consulUrl`: Consul server URL (default: http://127.0.0.1:8500)

## Test Coverage

Each request includes automated tests:

✅ **Status Code Validation**: Verifies correct HTTP status codes
✅ **Response Format**: Ensures JSON responses
✅ **Data Validation**: Checks response structure and types
✅ **Business Logic**: Verifies correct data transformations
✅ **Error Handling**: Tests error scenarios

### Test Results Example

```
✓ should return 200 status
✓ should return JSON response
✓ should return an array
✓ each pet should have required fields
```

## Running Tests via CLI

You can also run the collection via Bruno CLI:

```bash
# Install Bruno CLI
npm install -g @usebruno/cli

# Run the collection
bru run .bruno --env Local

# Run with output
bru run .bruno --env Local --output results.json

# Run specific folder
bru run .bruno/Pets --env Local
```

## Workflow Recommendations

### Complete Test Flow

Execute requests in this order:

1. **Health Check** - Verify service is up
2. **Get All Pets** - Should be empty initially
3. **Create Pet** - Creates first pet, saves ID
4. **Create Multiple Pets** - Adds more pets
5. **Get All Pets** - Should show created pets
6. **Get Pet by ID** - Retrieves specific pet
7. **Update Pet** - Modifies the pet
8. **Get Pet by ID** - Verify update
9. **Delete Pet** - Remove the pet
10. **Get Pet by ID** - Should return 404

### Error Testing Flow

1. **Get Nonexistent Pet** - Test 404
2. **Update Nonexistent Pet** - Test 404
3. **Delete Nonexistent Pet** - Test 404

### Consul Testing Flow

1. Start Consul: `consul agent -dev`
2. Start Pet Service: `npx nx run web_service`
3. **Verify Consul Registration** - Check service registered
4. **Check Pet Service Health** - Verify health monitoring

## Troubleshooting

### Service Not Responding

- Ensure pet service is running: `npx nx run web_service`
- Check service is on port 8080: `curl http://127.0.0.1:8080/health`

### Consul Tests Failing

- Start Consul: `consul agent -dev`
- Verify Consul is running: `curl http://127.0.0.1:8500/v1/agent/services`
- The service logs a warning if Consul is unavailable but continues running

### Variables Not Working

- Ensure you've run "Create Pet" before using `{{petId}}`
- Check that "Local" environment is selected
- Variables are stored per environment

### Tests Failing

- Check the "Tests" tab in Bruno for detailed error messages
- Verify the service is returning expected data format
- Ensure previous requests completed successfully (for chained requests)

## Advanced Usage

### Modifying Requests

Edit the `.bru` files directly in your IDE:

- Change request URLs
- Modify request bodies
- Update test assertions
- Add new headers

### Adding New Requests

1. Create new `.bru` file in appropriate folder
2. Follow the format from existing files
3. Add meta, request method, and test sections

### Environment Configuration

Create additional environments:

```
.bruno/environments/
├── Local.bru
├── Development.bru
└── Production.bru
```

### Custom Scripts

Add pre-request or post-response scripts:

```javascript
script:pre-request {
  // Set dynamic headers
  const timestamp = new Date().getTime();
  req.setHeader('X-Timestamp', timestamp);
}

script:post-response {
  // Save data for next request
  bru.setVar("token", res.body.token);
}
```

## Documentation

- **Bruno Docs**: https://docs.usebruno.com/
- **Bru Language**: https://docs.usebruno.com/bru-lang/overview
- **Pet Service API**: See `README_PROJECT.md` in project root

## Contributing

To add more tests:

1. Create new `.bru` file in appropriate folder
2. Follow naming convention: `Action Description.bru`
3. Include meta, request, docs, and tests sections
4. Update this README with the new test

## Summary

This collection provides:

- ✅ 12 API requests covering all endpoints
- ✅ 50+ automated test assertions
- ✅ Complete CRUD operation testing
- ✅ Error handling validation
- ✅ Consul integration verification
- ✅ Request chaining with variables
- ✅ Documentation for each endpoint

Perfect for:

- API testing during development
- CI/CD integration testing
- API documentation
- Onboarding new developers
- Regression testing
