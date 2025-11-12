use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod zkfile {
    use super::*;

    /// Initialize a new file account
    pub fn initialize_file(
        ctx: Context<InitializeFile>,
        file_id: String,
        cid: String,
        encrypted_key: Vec<u8>,
    ) -> Result<()> {
        instructions::initialize_file::handler(ctx, file_id, cid, encrypted_key)
    }

    /// Grant access to a file
    pub fn grant_access(
        ctx: Context<GrantAccess>,
        expires_at: Option<i64>,
    ) -> Result<()> {
        instructions::grant_access::handler(ctx, expires_at)
    }

    /// Revoke access to a file
    pub fn revoke_access(ctx: Context<RevokeAccess>) -> Result<()> {
        instructions::revoke_access::handler(ctx)
    }

    /// Verify a zero-knowledge proof
    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
    ) -> Result<()> {
        instructions::verify_proof::handler(ctx, proof, public_inputs)
    }

    /// Update file metadata
    pub fn update_file(
        ctx: Context<UpdateFile>,
        new_cid: Option<String>,
    ) -> Result<()> {
        instructions::update_file::handler(ctx, new_cid)
    }
}
