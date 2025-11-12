# ⚡ ZKFile Solana Program

> On-chain smart contracts for ZKFile Protocol

[![Anchor](https://img.shields.io/badge/Anchor-0.30.0-blueviolet)](https://www.anchor-lang.com/)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-1.18+-14F195?logo=solana)](https://solana.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

ZKFile Solana Program provides on-chain infrastructure for:
- **Access Control Lists (ACL)**: Granular permission management
- **Audit Trails**: Immutable activity logs
- **ZK-Proof Verification**: On-chain cryptographic verification
- **Metadata Storage**: Decentralized file metadata

## Program IDs

| Network | Program ID | Status |
|---------|-----------|--------|
| Mainnet-beta | `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS` | ✅ Live |
| Devnet | `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS` | ✅ Live |

## Installation

### Prerequisites
- Rust 1.75+
- Solana CLI 1.18+
- Anchor 0.30.0+

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.30.0
avm use 0.30.0
```

### Build

```bash
git clone https://github.com/zkfile/zkfile-solana-program
cd zkfile-solana-program
anchor build
```

### Test

```bash
anchor test
```

### Deploy

```bash
# Devnet
anchor deploy --provider.cluster devnet

# Mainnet
anchor deploy --provider.cluster mainnet
```

## Program Structure

```
programs/zkfile/
├── src/
│   ├── lib.rs              # Program entry point
│   ├── instructions/       # Instruction handlers
│   │   ├── initialize_file.rs
│   │   ├── grant_access.rs
│   │   ├── revoke_access.rs
│   │   ├── verify_proof.rs
│   │   └── update_file.rs
│   ├── state/              # Account structures
│   │   ├── file.rs
│   │   └── access.rs
│   └── errors.rs           # Custom errors
└── Cargo.toml
```

## Instructions

### 1. Initialize File

Creates a new file account with metadata.

```rust
pub fn initialize_file(
    ctx: Context<InitializeFile>,
    file_id: String,
    cid: String,
    encrypted_key: Vec<u8>,
) -> Result<()>
```

**Accounts:**
- `file`: File account (PDA)
- `owner`: File owner (signer)
- `system_program`: System program

### 2. Grant Access

Grants read access to another wallet.

```rust
pub fn grant_access(
    ctx: Context<GrantAccess>,
    expires_at: Option<i64>,
) -> Result<()>
```

**Accounts:**
- `file`: File account
- `access`: Access account (PDA)
- `owner`: File owner (signer)
- `recipient`: Recipient wallet
- `system_program`: System program

### 3. Revoke Access

Revokes access from a wallet.

```rust
pub fn revoke_access(
    ctx: Context<RevokeAccess>,
) -> Result<()>
```

**Accounts:**
- `file`: File account
- `access`: Access account
- `owner`: File owner (signer)

### 4. Verify ZK-Proof

Verifies a zero-knowledge proof on-chain.

```rust
pub fn verify_proof(
    ctx: Context<VerifyProof>,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<()>
```

### 5. Update File

Updates file metadata (CID).

```rust
pub fn update_file(
    ctx: Context<UpdateFile>,
    new_cid: Option<String>,
) -> Result<()>
```

## Account Structures

### File Account

```rust
#[account]
pub struct File {
    pub owner: Pubkey,              // File owner
    pub file_id: String,            // Unique file identifier
    pub cid: String,                // IPFS/Arweave CID
    pub encrypted_key: Vec<u8>,     // Encrypted symmetric key
    pub created_at: i64,            // Creation timestamp
    pub updated_at: i64,            // Last update timestamp
    pub access_count: u32,          // Number of access grants
    pub bump: u8,                   // PDA bump seed
}
```

### Access Account

```rust
#[account]
pub struct Access {
    pub file: Pubkey,               // File account
    pub recipient: Pubkey,          // Recipient wallet
    pub granted_by: Pubkey,         // Grantor wallet
    pub granted_at: i64,            // Grant timestamp
    pub expires_at: Option<i64>,    // Expiration timestamp
    pub revoked: bool,              // Revocation status
    pub bump: u8,                   // PDA bump seed
}
```

## Security

### Audits
- ✅ Trail of Bits - Q4 2024
- ✅ Kudelski Security - Q1 2025
- ✅ OtterSec - Q1 2025

### Security Features
- PDA-based access control
- Time-locked permissions
- Immutable audit trails
- Cryptographic proof verification
- Rent-exempt accounts

### Bug Bounty
Up to $100,000 for critical vulnerabilities  
Report to: security@zkfile.tech

## Gas Costs

| Instruction | Compute Units | SOL Cost |
|-------------|---------------|----------|
| Initialize File | ~15,000 | ~0.00001 |
| Grant Access | ~8,000 | ~0.000005 |
| Revoke Access | ~5,000 | ~0.000003 |
| Verify Proof | ~25,000 | ~0.00002 |
| Update File | ~6,000 | ~0.000004 |

## Development

### Local Testing

```bash
# Start local validator
solana-test-validator

# Deploy locally
anchor deploy

# Run tests
anchor test --skip-local-validator
```

### Integration with SDK

```typescript
import { ZKFileClient } from '@zkfile/core-sdk';
import { Connection, clusterApiUrl } from '@solana/web3.js';

const zkfile = new ZKFileClient({
  rpcEndpoint: clusterApiUrl('devnet'),
  programId: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
});

// Upload file (calls initialize_file instruction)
const result = await zkfile.upload({
  file,
  password: 'secure-password',
  wallet
});
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md)

```bash
git clone https://github.com/zkfile/zkfile-solana-program
cd zkfile-solana-program
anchor build
anchor test
```

## License

MIT © 2025 ZKFile Labs

## Links

- 🌐 [Website](https://zkfile.tech)
- 📚 [Whitepaper](https://paper.zkfile.tech)
- 🐦 [X](https://twitter.com/ZKFile_Tech)
- 📧 [Email](mailto:team@zkfile.tech)

---

Built with ⚡ on Solana using Anchor
