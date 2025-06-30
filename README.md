# Solana REST API Server

A lightweight HTTP server built with Rust and Axum that provides Solana-related REST API endpoints. This server simulates Solana operations locally using the Solana SDK without requiring actual blockchain interaction.

## Features

- **Keypair Generation**: Generate new Solana keypairs
- **Token Operations**: Create SPL token mint and mint_to instructions
- **Message Signing**: Sign messages with Ed25519 private keys
- **Message Verification**: Verify Ed25519 signatures
- **Consistent API**: All endpoints follow a consistent JSON response format

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

The server will start on `http://localhost:3000` by default.

### Running Tests

```bash
cargo test
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
    "public_key": "base58-encoded-public-key",
    "secret_key": "base58-encoded-secret-key"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/keypair
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
curl -X POST http://localhost:3000/token/create \
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
curl -X POST http://localhost:3000/token/mint \
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

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base64-encoded-signature",
    "public_key": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/message/sign \
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
  "signature": "base64-encoded-signature",
  "public_key": "base58-encoded-public-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/message/verify \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","signature":"base64-signature","public_key":"base58-public-key"}'
```

## Example Workflow

Here's a complete example of using all endpoints together:

1. **Generate a keypair:**
```bash
curl -X POST http://localhost:3000/keypair
# Returns: {"success":true,"data":{"public_key":"AbC...","secret_key":"XyZ..."}}
```

2. **Sign a message:**
```bash
curl -X POST http://localhost:3000/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","secret":"XyZ..."}'
# Returns: {"success":true,"data":{"signature":"ABC...","public_key":"AbC...","message":"Hello, Solana!"}}
```

3. **Verify the signature:**
```bash
curl -X POST http://localhost:3000/message/verify \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Solana!","signature":"ABC...","public_key":"AbC..."}'
# Returns: {"success":true,"data":{"valid":true}}
```

4. **Create a token mint instruction:**
```bash
curl -X POST http://localhost:3000/token/create \
  -H "Content-Type: application/json" \
  -d '{"mintAuthority":"AbC...","mint":"DeF...","decimals":9}'
```

## Architecture

The server is built with:

- **Axum**: Modern, ergonomic web framework
- **Solana SDK**: For keypair generation and cryptographic operations
- **SPL Token**: For token instruction generation
- **Ed25519-Dalek**: For signature verification
- **Tokio**: Async runtime

The code is organized into modules:

- `handlers/`: HTTP request handlers
- `models/`: Request/response data structures  
- `services/`: Business logic (Solana operations)
- `errors/`: Error handling and types
- `router/`: Route configuration

## Error Handling

The API provides detailed error messages for common issues:

- Invalid public key format
- Invalid secret key format
- Missing required fields
- Invalid signature format
- Signature verification failures

## Development

### Project Structure
```
src/
├── main.rs           # Application entry point
├── router/           # Route definitions
├── handlers/         # Request handlers
├── services/         # Business logic
├── models/           # Data structures
└── errors/           # Error handling
```

### Adding New Endpoints

1. Define request/response models in `src/models.rs`
2. Implement business logic in `src/services/`
3. Create handler function in `src/handlers/`  
4. Add route in `src/router/mod.rs`

## Security Notes

⚠️ **Important**: This server is designed for development and testing purposes. In production:

- Never log or expose secret keys
- Use HTTPS in production
- Implement proper authentication
- Add rate limiting
- Validate all inputs thoroughly

## License

This project is provided as-is for educational and development purposes. 
