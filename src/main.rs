use blockchainlib::*;
fn main() {
    let difficulty = 0x000000ffffffffffffffffffffffffff;
    let coinbase_reward = 100;

    let mut blockchain = Blockchain::new(difficulty, coinbase_reward);


    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );
    println!("{:?}", &genesis_block);

    genesis_block.mine();
    println!("Mined Genesis {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();


    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 10,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 0,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 48,
                    },
                ],
            }
        ],
        difficulty,
    );
    println!("{:?}", &block);

    block.mine();
    println!("Mined {:?}", &block);

    last_hash = block.hash.clone();

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");



    let mut block2 = Block::new(
        2,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Stef".to_owned(),
                    value: 10,
                }],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[1].clone(),
                    blockchain.blocks[1].transactions[1].outputs[1].clone()
                    ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 2,
                    },
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 17,
                    },
                ],
            },
        ],
        difficulty,
    );
    println!("{:?}", &block2);

    block2.mine();
    println!("Mined {:?}", &block2);

    last_hash = block2.hash.clone();

    blockchain
        .update_with_block(block2)
        .expect("Failed to add block");


    println!("UTXO: {:?}", blockchain.unspent_outputs);

    // for i in 1..=10 {
    //     let mut block = Block::new(i, now(), last_hash, 0, "New block".to_owned(), difficulty);
    //     println!("{:?}", &block);

    //     block.mine();
    //     println!("Mined {:?}", &block);

    //     last_hash = block.hash.clone();

    //     blockchain.blocks.push(block);
    //     println!("Verify {:?}", &blockchain.update_with_block());
    // }

    // println!("Verify {:?}", &blockchain.update_with_block());
}
