# Solana REST API Server

A lightweight HTTP server built with Rust and Axum that provides Solana-related REST API endpoints. This server simulates Solana operations locally using the Solana SDK without requiring actual blockchain interaction.

## Features

- **Keypair Generation**: Generate new Solana keypairs
- **Token Operations**: Create SPL token mint and mint_to instructions
- **Message Signing**: Sign messages with Ed25519 private keys
- **Message Verification**: Verify Ed25519 signatures
- **SOL Transfers**: Create SOL transfer instructions
- **Token Transfers**: Create SPL token transfer instructions
- **Comprehensive Validation**: Input validation with detailed error messages
- **Consistent API**: All endpoints follow a consistent JSON response format
- **Extensive Testing**: 30+ unit tests covering all functionality

## Getting Started

### Prerequisites

- Rust 1.70+ (with Cargo)
- Git

### Installation & Running

1. Clone the repository:
```bash
git clone <repository_url>
cd simple-rest-solana
```

2. Install dependencies:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:8080` by default and will display:
```
Available endpoints:
  POST /keypair         - Generate new Solana keypair
  POST /token/create    - Create SPL token mint instruction
  POST /token/mint      - Create SPL token mint_to instruction
  POST /message/sign    - Sign message with secret key
  POST /message/verify  - Verify message signature
  POST /send/sol        - Create SOL transfer instruction
  POST /send/token      - Create SPL token transfer instruction
```

### Running Tests

Run all tests (30+ test cases):
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run specific test module:
```bash
cargo test handlers::tests
cargo test utils::validation::tests
cargo test services::solana::tests
```

## API Endpoints

All endpoints return JSON responses in the following format:

**Success Response:**
```json
{
  "success": true,
  "data": { /* endpoint-specific data */ }
}
```

**Error Response:**
```json
{
  "success": false,
  "error": "Description of error"
}
```

### 1. Generate Keypair

**POST** `/keypair`

Generates a new Solana keypair.

**Response:**
```json
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/keypair
```

### 2. Create Token Mint

**POST** `/token/create`

Creates an SPL token mint instruction.

**Request Body:**
```json
{
  "mintAuthority": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key", 
  "decimals": 9
}
```

**Validation:**
- `mintAuthority`: Required, valid Solana public key
- `mint`: Required, valid Solana public key
- `decimals`: Must be between 0 and 9

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "base58-encoded-public-key",
        "is_signer": false,
        "is_writable": true
      }
    ],
    "instruction_data": "base64-encoded-instruction-bytes"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/token/create \
  -H "Content-Type: application/json" \
  -d '{"mintAuthority":"11111111111111111111111111111112","mint":"11111111111111111111111111111113","decimals":9}'
```

### 3. Mint Tokens

**POST** `/token/mint`

Creates an SPL token mint_to instruction.

**Request Body:**
```json
{
  "mint": "base58-encoded-mint-address",
  "destination": "base58-encoded-destination-address",
  "authority": "base58-encoded-authority-address",
  "amount": 1000000000
}
```

**Validation:**
- `mint`: Required, valid Solana public key
- `destination`: Required, valid Solana public key
- `authority`: Required, valid Solana public key
- `amount`: Must be greater than 0

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "base58-encoded-public-key",
        "is_signer": false,
        "is_writable": true
      }
    ],
    "instruction_data": "base64-encoded-instruction-bytes"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/token/mint \
  -H "Content-Type: application/json" \
  -d '{"mint":"11111111111111111111111111111113","destination":"11111111111111111111111111111112","authority":"11111111111111111111111111111112","amount":1000000000}'
```

### 4. Sign Message

**POST** `/message/sign`

Signs a message using an Ed25519 private key.

**Request Body:**
```json
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```

**Validation:**
- `message`: Required, non-empty string
- `secret`: Required, valid base58-encoded 64-byte secret key

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base58-encoded-signature",
    "pubkey": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","secret":"your-base58-secret-key"}'
```

### 5. Verify Message

**POST** `/message/verify`

Verifies an Ed25519 signature against a message and public key.

**Request Body:**
```json
{
  "message": "Hello, Solana!",
  "signature": "base58-encoded-signature",
  "pubkey": "base58-encoded-public-key"
}
```

**Validation:**
- `message`: Required, non-empty string
- `signature`: Required, valid base58-encoded 64-byte signature
- `pubkey`: Required, valid Solana public key

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "base58-encoded-public-key"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/message/verify \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","signature":"base58-signature","pubkey":"base58-public-key"}'
```

### 6. Send SOL

**POST** `/send/sol`

Creates a SOL transfer instruction.

**Request Body:**
```json
{
  "from": "base58-encoded-sender-pubkey",
  "to": "base58-encoded-recipient-pubkey",
  "lamports": 1000000
}
```

**Validation:**
- `from`: Required, valid Solana public key
- `to`: Required, valid Solana public key
- `lamports`: Must be greater than 0

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "11111111111111111111111111111111",
    "accounts": [
      "base58-encoded-sender-pubkey",
      "base58-encoded-recipient-pubkey"
    ],
    "instruction_data": "base64-encoded-instruction-bytes"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/send/sol \
  -H "Content-Type: application/json" \
  -d '{"from":"sender-pubkey","to":"recipient-pubkey","lamports":1000000}'
```

### 7. Send Token

**POST** `/send/token`

Creates an SPL token transfer instruction.

**Request Body:**
```json
{
  "destination": "base58-encoded-destination-pubkey",
  "mint": "base58-encoded-mint-address",
  "owner": "base58-encoded-owner-pubkey",
  "amount": 1000000
}
```

**Validation:**
- `destination`: Required, valid Solana public key
- `mint`: Required, valid Solana public key
- `owner`: Required, valid Solana public key
- `amount`: Must be greater than 0

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "base58-encoded-pubkey",
        "isSigner": false
      }
    ],
    "instruction_data": "base64-encoded-instruction-bytes"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/send/token \
  -H "Content-Type: application/json" \
  -d '{"destination":"dest-pubkey","mint":"mint-address","owner":"owner-pubkey","amount":1000000}'
```

## Example Workflow

Here's a complete example of using all endpoints together:

1. **Generate a keypair:**
```bash
curl -X POST http://localhost:8080/keypair
# Returns: {"success":true,"data":{"pubkey":"AbC...","secret":"XyZ..."}}
```

2. **Sign a message:**
```bash
curl -X POST http://localhost:8080/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","secret":"XyZ..."}'
# Returns: {"success":true,"data":{"signature":"ABC...","pubkey":"AbC...","message":"Hello, Solana!"}}
```

3. **Verify the signature:**
```bash
curl -X POST http://localhost:8080/message/verify \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","signature":"ABC...","pubkey":"AbC..."}'
# Returns: {"success":true,"data":{"valid":true,"message":"Hello, Solana!","pubkey":"AbC..."}}
```

4. **Create a token mint instruction:**
```bash
curl -X POST http://localhost:8080/token/create \
  -H "Content-Type: application/json" \
  -d '{"mintAuthority":"AbC...","mint":"DeF...","decimals":9}'
```

5. **Create a SOL transfer:**
```bash
curl -X POST http://localhost:8080/send/sol \
  -H "Content-Type: application/json" \
  -d '{"from":"AbC...","to":"DeF...","lamports":1000000}'
```

## Architecture

The server is built with:

- **Axum**: Modern, ergonomic web framework
- **Solana SDK**: For keypair generation and cryptographic operations
- **SPL Token**: For token instruction generation
- **Ed25519-Dalek**: For signature verification
- **Tokio**: Async runtime
- **Tracing**: Structured logging

## Project Structure

```
src/
├── main.rs              # Application entry point
├── router/             
│   └── mod.rs           # Route definitions and middleware
├── handlers/           
│   └── mod.rs           # Request handlers with validation
├── services/           
│   ├── mod.rs           # Business logic modules
│   └── solana.rs        # Solana operations (447 lines)
├── models/             
│   └── mod.rs           # Request/response data structures
└── utils/              
    ├── mod.rs           # Utility modules
    ├── errors.rs        # Error handling and types
    └── validation.rs    # Input validation functions
```

### Module Responsibilities

- **`router/`**: HTTP routing, middleware, request logging
- **`handlers/`**: Request/response handling, input validation, error handling
- **`services/`**: Core business logic for Solana operations
- **`models/`**: Serde-compatible data structures for JSON serialization
- **`utils/errors`**: Centralized error handling with proper HTTP status codes
- **`utils/validation`**: Input validation functions with detailed error messages

## Testing

The project includes comprehensive testing with **30+ test cases** covering:

### Test Categories

1. **Unit Tests** (`utils/`):
   - Error handling and HTTP response generation
   - Input validation (public keys, amounts, signatures)
   - Edge cases and error conditions

2. **Integration Tests** (`handlers/`):
   - Full request/response cycle testing
   - JSON serialization/deserialization
   - Error response formatting

3. **Service Tests** (`services/`):
   - Solana SDK integration
   - Keypair generation
   - Message signing and verification
   - Instruction creation

### Running Tests

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test modules
cargo test utils::validation::tests
cargo test handlers::tests  
cargo test services::solana::tests

# Run tests with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Test Output Example
```
running 30 tests
test models::tests::test_api_response_serialization ... ok
test handlers::tests::test_generate_keypair_handler ... ok
test services::solana::tests::test_generate_keypair ... ok
test utils::validation::tests::test_validate_pubkey_valid ... ok
...
test result: ok. 30 passed; 0 failed; 0 ignored
```

## Error Handling

The API provides detailed error messages for common issues:

- **400 Bad Request**: Invalid input data, malformed keys, validation failures
- **500 Internal Server Error**: Server-side processing errors

### Common Error Responses

```json
{
  "success": false,
  "error": "Invalid public key: not valid base58"
}
```

```json
{
  "success": false,
  "error": "Amount must be greater than 0"
}
```

```json
{
  "success": false,
  "error": "Signature verification failed"
}
```

## Development

### Adding New Endpoints

1. Define request/response models in `src/models/mod.rs`
2. Add validation functions in `src/utils/validation.rs`
3. Implement business logic in `src/services/`
4. Create handler function in `src/handlers/mod.rs`
5. Add route in `src/router/mod.rs`
6. Write comprehensive tests

### Code Quality

The project maintains high code quality with:
- Comprehensive error handling
- Input validation
- Unit and integration tests
- Structured logging
- Clear separation of concerns
- Detailed documentation

## Performance

- **Async/await**: Non-blocking I/O operations
- **Zero-copy serialization**: Efficient JSON handling
- **Minimal dependencies**: Fast compilation and small binary
- **Request logging**: Performance monitoring capabilities

## Security Notes

⚠️ **Important**: This server is designed for development and testing purposes. In production:

- Never log or expose secret keys
- Use HTTPS in production
- Implement proper authentication and authorization
- Add rate limiting and DDoS protection
- Validate all inputs thoroughly
- Use secure key storage solutions
- Implement proper CORS policies
- Add request size limits

## License

This project is provided as-is for educational and development purposes.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add comprehensive tests
4. Ensure all tests pass: `cargo test`
5. Update documentation
6. Submit a pull request

## Changelog

### v0.1.0
- Initial release with 7 endpoints
- Comprehensive validation and error handling
- 30+ unit and integration tests
- Structured project organization
- Detailed API documentation
