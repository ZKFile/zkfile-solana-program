use anchor_lang::prelude::*;

#[account]
pub struct File {
    /// File owner
    pub owner: Pubkey,
    /// Unique file identifier
    pub file_id: String,
    /// Content identifier (IPFS/Arweave CID)
    pub cid: String,
    /// Encrypted symmetric key
    pub encrypted_key: Vec<u8>,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Number of access grants
    pub access_count: u32,
    /// Bump seed for PDA
    pub bump: u8,
}

impl File {
    pub const MAX_FILE_ID_LEN: usize = 64;
    pub const MAX_CID_LEN: usize = 128;
    pub const MAX_ENCRYPTED_KEY_LEN: usize = 256;

    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        4 + Self::MAX_FILE_ID_LEN + // file_id
        4 + Self::MAX_CID_LEN + // cid
        4 + Self::MAX_ENCRYPTED_KEY_LEN + // encrypted_key
        8 + // created_at
        8 + // updated_at
        4 + // access_count
        1; // bump
}
