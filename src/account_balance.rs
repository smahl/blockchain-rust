use crate::{transaction::Output, Blockchain};


pub struct AccountBalance {
    name: String,
    balance: u64,
    utxos: Vec<UTXO>
}

pub struct UTXO {
    account_name: String,
    value: Output,
    block_index: u8,
    transaction_index: u8,
    output_index: u8

}


// impl AccountBalance {
//     pub fn new (    
//     name: String,
//     blockchain: Blockchain) -> Self {

//     }
// }