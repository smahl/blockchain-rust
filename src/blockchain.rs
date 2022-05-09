use std::collections::{HashMap, HashSet};

use crate::account_balance::UTXO;

use super::*;

#[derive(Debug)]
pub enum BlockValidationErr {
    MisatchedIndex,
    InvalidHash,
    AcronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub unspent_outputs: HashSet<Hash>,
    pub unspent_outputs_info: HashMap<Hash, UTXO>,
    difficulty: u128,
    coinbase_reward: u64,
}

impl Blockchain {
    pub fn new(difficulty: u128, coinbase_reward: u64) -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
            unspent_outputs_info: HashMap::new(),
            difficulty,
            coinbase_reward,
        }
    }
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();
        if block.index != i as u32 {
            println!("Index mistmatch {} != {}", &block.index, &i);
            return Err(BlockValidationErr::MisatchedIndex);
        } else if !block::check_difficulty(&block.hash(), self.difficulty) {
            println!("Difficulty failed");
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // Regular block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                println!("Time did not increase");
                return Err(BlockValidationErr::AcronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                println!("Hash mismatch, previous hash not equal to hash of previous block ");
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                println!("Genesis block prev hash invalid");
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            println!("{}", coinbase.outputs[0].to_addr);
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            if coinbase.output_value() > self.coinbase_reward {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                let input_hashes_subtract_unspent_output = &input_hashes - &self.unspent_outputs;
                let join_input_hashes_block_spent = &input_hashes & &block_spent;
                if !(input_hashes_subtract_unspent_output).is_empty()
                    || !(join_input_hashes_block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() > self.coinbase_reward + total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs_info
                .retain(|output, _| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
            self.unspent_outputs_info.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}
