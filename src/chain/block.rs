// Yes, I acknowledge the merkle root isn't actually the merkle root right now.

use sha2::{Digest, Sha256};
use std::fmt;

use crate::chain::transaction::Transaction;

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub nonce: u64,
    pub data: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, prev_hash: [u8; 32], data: [u8; 1024]) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut hasher = Sha256::new();
        hasher.update(data);
        let merkle_root = hasher.finalize().into();

        Self {
            index,
            timestamp,
            prev_hash,
            merkle_root,
            nonce: 0,
            data: Vec::new(),
        }
    }

    pub fn create_genesis() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            index: 0,
            timestamp,
            prev_hash: [0; 32],
            merkle_root: [0; 32],
            nonce: 0,
            data: Vec::new(),
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.prev_hash);
        hasher.update(self.merkle_root);
        hasher.update(self.nonce.to_le_bytes());
        // hasher.update(self.data);
        let hash: [u8; 32] = hasher.finalize().into();
        return hash;
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block #{}", self.index)?;
        writeln!(f, "  Timestamp:        {}", self.timestamp)?;
        writeln!(f, "  Prev Hash:        {}", hex::encode(self.prev_hash))?;
        writeln!(f, "  Merkle Root:      {}", hex::encode(self.merkle_root))?;
        writeln!(f, "  Nonce:            {}", self.nonce)?;
        writeln!(f, "  Num Transactions: {}", self.data.len())
    }
}
