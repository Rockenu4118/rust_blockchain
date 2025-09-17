use std::collections::HashMap;

use crate::{chain::block::Block, chain::transaction::Transaction};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub balances: HashMap<[u8; 33], i64>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::create_genesis();

        Self {
            chain: vec![genesis],
            mempool: Vec::new(),
            balances: HashMap::new(),
            difficulty,
        }
    }

    // Return a reference to the block at the tip of the chain
    pub fn tip(&self) -> &Block {
        self.chain.last().expect("chain not empty")
    }

    // Add a transaction to the mempool
    pub fn add_transaction(&mut self, tx: Transaction) -> () {
        self.mempool.push(tx);
    }

    // Process a block once it has been mined and being added to chain
    pub fn add_block(&mut self, block: Block) -> () {
        for tx in &block.data {
            {
                let recipient_balance = self.balances.entry(tx.recipient).or_insert(0);
                *recipient_balance += tx.amount as i64;
            }
            let sender_balance = self.balances.entry(tx.sender).or_insert(0);
            *sender_balance -= tx.amount as i64;
        }
    }

    // Mine a block
    pub fn mine_block(&mut self) -> () {
        let prev_block = self.tip();

        let mut new_block = Block::new(prev_block.index + 1, prev_block.hash(), [0; 1024]);

        for tx in &self.mempool {
            new_block.data.push(tx.clone());
        }

        self.mempool.clear();

        loop {
            let new_block_hash = hex::encode(new_block.hash());
            let target_prefix = "0".repeat(self.difficulty);

            if new_block_hash.starts_with(target_prefix.as_str()) {
                break;
            } else {
                new_block.nonce += 1;
            }
        }

        self.add_block(new_block);
    }
}
