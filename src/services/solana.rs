use std::str::FromStr;

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer, Signature},
    instruction::Instruction,
    system_instruction,
};
use spl_token::{
    instruction::{initialize_mint, mint_to, transfer},
};
use bs58;
use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::{Verifier, PublicKey as Ed25519PublicKey, ed25519::signature::Signature as Ed25519Signature};

use crate::models::{
    KeypairResponse, 
    TokenInstructionResponse, 
    AccountMeta, 
    SignMessageResponse, 
    VerifyMessageResponse,
    SendSolResponse,
    SendTokenResponse,
    SendTokenAccountMeta,
};
use crate::errors::{AppError, Result, base58_decode_error, base64_decode_error};

/// Solana service for interacting with the Solana blockchain
pub struct SolanaService;

impl SolanaService {
    /// Creates a new SolanaService instance
    pub fn new() -> Self {
        Self
    }

    /// Generates a new Solana keypair
    pub fn generate_keypair(&self) -> Result<KeypairResponse> {
        let keypair = Keypair::new();
        
        let pubkey = keypair.pubkey().to_string();
        let secret = bs58::encode(&keypair.to_bytes()).into_string();

        Ok(KeypairResponse {
            pubkey,
            secret,
        })
    }

    /// Creates an SPL token mint instruction
    pub fn create_token_mint(
        &self,
        mint_authority: &str,
        mint: &str,
        decimals: u8,
    ) -> Result<TokenInstructionResponse> {
        // Parse public keys
        let mint_authority_pubkey = Pubkey::from_str(mint_authority)
            .map_err(|_| AppError::InvalidPublicKey(mint_authority.to_string()))?;
        
        let mint_pubkey = Pubkey::from_str(mint)
            .map_err(|_| AppError::InvalidPublicKey(mint.to_string()))?;

        // Create the initialize_mint instruction
        let instruction = initialize_mint(
            &spl_token::id(),
            &mint_pubkey,
            &mint_authority_pubkey,
            Some(&mint_authority_pubkey), // freeze authority (same as mint authority)
            decimals,
        ).map_err(|e| AppError::TokenOperationFailed(e.to_string()))?;

        self.instruction_to_response(instruction)
    }

    /// Creates an SPL token mint_to instruction
    pub fn mint_token(
        &self,
        mint: &str,
        destination: &str,
        authority: &str,
        amount: u64,
    ) -> Result<TokenInstructionResponse> {
        // Parse public keys
        let mint_pubkey = Pubkey::from_str(mint)
            .map_err(|_| AppError::InvalidPublicKey(mint.to_string()))?;
        
        let destination_pubkey = Pubkey::from_str(destination)
            .map_err(|_| AppError::InvalidPublicKey(destination.to_string()))?;
        
        let authority_pubkey = Pubkey::from_str(authority)
            .map_err(|_| AppError::InvalidPublicKey(authority.to_string()))?;

        // Create the mint_to instruction
        let instruction = mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &destination_pubkey,
            &authority_pubkey,
            &[],
            amount,
        ).map_err(|e| AppError::TokenOperationFailed(e.to_string()))?;

        self.instruction_to_response(instruction)
    }

    /// Signs a message with the provided secret key
    pub fn sign_message(
        &self,
        message: &str,
        secret_key: &str,
    ) -> Result<SignMessageResponse> {
        // Decode the secret key from base58
        let secret_bytes = bs58::decode(secret_key)
            .into_vec()
            .map_err(base58_decode_error)?;

        // Create keypair from secret key bytes
        let keypair = Keypair::from_bytes(&secret_bytes)
            .map_err(|_| AppError::InvalidSecretKey("Invalid secret key format".to_string()))?;

        // Sign the message
        let message_bytes = message.as_bytes();
        let signature = keypair.sign_message(message_bytes);

        // Encode signature as base64
        let signature_base64 = general_purpose::STANDARD.encode(signature.as_ref());

        Ok(SignMessageResponse {
            signature: signature_base64,
            public_key: keypair.pubkey().to_string(),
            message: message.to_string(),
        })
    }

    /// Verifies a message signature
    pub fn verify_message(
        &self,
        message: &str,
        signature_base64: &str,
        pubkey: &str,
    ) -> Result<VerifyMessageResponse> {
        // Decode signature from base64
        let signature_bytes = general_purpose::STANDARD
            .decode(signature_base64)
            .map_err(base64_decode_error)?;

        // Parse public key
        let pubkey_parsed = Pubkey::from_str(pubkey)
            .map_err(|_| AppError::InvalidPublicKey(pubkey.to_string()))?;

        // Create signature from bytes
        let signature = Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| AppError::InvalidSignature("Invalid signature format".to_string()))?;

        // Verify using ed25519-dalek for compatibility
        let message_bytes = message.as_bytes();
        let valid = self.verify_ed25519_signature(&pubkey_parsed, message_bytes, &signature)?;

        Ok(VerifyMessageResponse {
            valid,
            message: message.to_string(),
            pubkey: pubkey.to_string(),
        })
    }

    /// Creates a SOL transfer instruction
    pub fn send_sol(
        &self,
        from: &str,
        to: &str,
        lamports: u64,
    ) -> Result<SendSolResponse> {
        // Parse public keys
        let from_pubkey = Pubkey::from_str(from)
            .map_err(|_| AppError::InvalidPublicKey(from.to_string()))?;
        
        let to_pubkey = Pubkey::from_str(to)
            .map_err(|_| AppError::InvalidPublicKey(to.to_string()))?;

        // Validate that lamports is greater than 0
        if lamports == 0 {
            return Err(AppError::ValidationError("Amount must be greater than 0".to_string()));
        }

        // Create the transfer instruction
        let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);

        // Convert to simplified response format for SOL transfers
        Ok(SendSolResponse {
            program_id: instruction.program_id.to_string(),
            accounts: vec![from.to_string(), to.to_string()],
            instruction_data: general_purpose::STANDARD.encode(&instruction.data),
        })
    }

    /// Creates an SPL token transfer instruction
    pub fn send_token(
        &self,
        destination: &str,
        mint: &str,
        owner: &str,
        amount: u64,
    ) -> Result<SendTokenResponse> {
        // Parse public keys
        let destination_pubkey = Pubkey::from_str(destination)
            .map_err(|_| AppError::InvalidPublicKey(destination.to_string()))?;
        
        let mint_pubkey = Pubkey::from_str(mint)
            .map_err(|_| AppError::InvalidPublicKey(mint.to_string()))?;
        
        let owner_pubkey = Pubkey::from_str(owner)
            .map_err(|_| AppError::InvalidPublicKey(owner.to_string()))?;

        // Validate amount
        if amount == 0 {
            return Err(AppError::ValidationError("Amount must be greater than 0".to_string()));
        }

        // For token transfers, we need source and destination token accounts
        // We'll use associated token account addresses (simplified)
        let source_ata = spl_associated_token_account::get_associated_token_address(&owner_pubkey, &mint_pubkey);
        let dest_ata = spl_associated_token_account::get_associated_token_address(&destination_pubkey, &mint_pubkey);

        // Create the transfer instruction
        let instruction = transfer(
            &spl_token::id(),
            &source_ata,
            &dest_ata,
            &owner_pubkey,
            &[],
            amount,
        ).map_err(|e| AppError::TokenOperationFailed(e.to_string()))?;

        // Convert accounts to the specific format for send token endpoint
        let accounts: Vec<SendTokenAccountMeta> = instruction.accounts
            .into_iter()
            .map(|acc| SendTokenAccountMeta {
                pubkey: acc.pubkey.to_string(),
                is_signer: acc.is_signer,
            })
            .collect();

        Ok(SendTokenResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(&instruction.data),
        })
    }

    /// Helper function to convert Solana Instruction to our response format
    fn instruction_to_response(&self, instruction: Instruction) -> Result<TokenInstructionResponse> {
        // Convert accounts
        let accounts: Vec<AccountMeta> = instruction.accounts
            .into_iter()
            .map(|acc| AccountMeta {
                pubkey: acc.pubkey.to_string(),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect();

        // Encode instruction data as base64
        let instruction_data = general_purpose::STANDARD.encode(&instruction.data);

        Ok(TokenInstructionResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data,
        })
    }

    /// Helper function to verify ed25519 signature using ed25519-dalek
    fn verify_ed25519_signature(
        &self,
        pubkey: &Pubkey,
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool> {
        // Convert Solana pubkey to ed25519-dalek public key
        let ed25519_pubkey = Ed25519PublicKey::from_bytes(pubkey.as_ref())
            .map_err(|_| AppError::InvalidPublicKey("Invalid public key for verification".to_string()))?;

        // Convert Solana signature to ed25519-dalek signature
        let ed25519_signature = Ed25519Signature::from_bytes(signature.as_ref())
            .map_err(|_| AppError::InvalidSignature("Invalid signature format".to_string()))?;

        // Verify the signature
        let is_valid = ed25519_pubkey.verify(message, &ed25519_signature).is_ok();
        
        Ok(is_valid)
    }

    /// Validates if a string is a valid base58-encoded Solana public key
    pub fn is_valid_pubkey(&self, pubkey_str: &str) -> bool {
        Pubkey::from_str(pubkey_str).is_ok()
    }

    /// Validates if a string is a valid base58-encoded Solana secret key
    pub fn is_valid_secret_key(&self, secret_key_str: &str) -> bool {
        match bs58::decode(secret_key_str).into_vec() {
            Ok(bytes) => bytes.len() == 64, // Solana secret keys are 64 bytes
            Err(_) => false,
        }
    }
}

impl Default for SolanaService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let service = SolanaService::new();
        let result = service.generate_keypair();
        
        assert!(result.is_ok());
        let keypair_response = result.unwrap();
        assert!(!keypair_response.pubkey.is_empty());
        assert!(!keypair_response.secret.is_empty());
    }

    #[test]
    fn test_sign_and_verify_message() {
        let service = SolanaService::new();
        
        // Generate a keypair
        let keypair_response = service.generate_keypair().unwrap();
        let message = "Hello, Solana!";
        
        // Sign the message
        let sign_result = service.sign_message(message, &keypair_response.secret);
        assert!(sign_result.is_ok());
        
        let sign_response = sign_result.unwrap();
        assert_eq!(sign_response.message, message);
        assert_eq!(sign_response.public_key, keypair_response.pubkey);
        
        // Verify the signature
        let verify_result = service.verify_message(
            message, 
            &sign_response.signature, 
            &sign_response.public_key
        );
        assert!(verify_result.is_ok());
        let verify_response = verify_result.unwrap();
        assert!(verify_response.valid);
        assert_eq!(verify_response.message, message);
    }

    #[test]
    fn test_invalid_signature_verification() {  
        let service = SolanaService::new();
        
        let keypair_response = service.generate_keypair().unwrap();
        // Create a valid base64 string with the correct length for a signature (64 bytes)
        let invalid_signature = general_purpose::STANDARD.encode(&[0u8; 64]);
        
        let verify_result = service.verify_message(
            "test message",
            &invalid_signature,
            &keypair_response.pubkey
        );
        
        // Should succeed but return valid: false
        assert!(verify_result.is_ok());
        assert!(!verify_result.unwrap().valid);
    }

    #[test]
    fn test_pubkey_validation() {
        let service = SolanaService::new();
        
        // Valid pubkey
        assert!(service.is_valid_pubkey("11111111111111111111111111111112"));
        
        // Invalid pubkey
        assert!(!service.is_valid_pubkey("invalid_pubkey"));
        assert!(!service.is_valid_pubkey(""));
    }

    #[test]
    fn test_token_mint_instruction() {
        let service = SolanaService::new();
        
        let mint_authority = "11111111111111111111111111111112";
        let mint = "11111111111111111111111111111113";
        
        let result = service.create_token_mint(mint_authority, mint, 9);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.program_id, spl_token::id().to_string());
        assert!(!response.accounts.is_empty());
        assert!(!response.instruction_data.is_empty());
    }

    #[test]
    fn test_send_sol_instruction() {
        let service = SolanaService::new();
        
        let from = "11111111111111111111111111111112";
        let to = "11111111111111111111111111111113";
        let lamports = 1000000;
        
        let result = service.send_sol(from, to, lamports);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.program_id, solana_sdk::system_program::id().to_string());
        assert_eq!(response.accounts.len(), 2);
        assert!(!response.instruction_data.is_empty());
    }

    #[test]
    fn test_send_sol_zero_amount() {
        let service = SolanaService::new();
        
        let from = "11111111111111111111111111111112";
        let to = "11111111111111111111111111111113";
        
        let result = service.send_sol(from, to, 0);
        assert!(result.is_err());
    }
} 
