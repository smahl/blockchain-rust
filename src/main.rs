use blockchainlib::*;
fn main () {
    let mut block = Block::new(13,12345,vec![0; 32], 0, "Genesis block!".to_owned(), 0x000fffffffffffffffffffffffffffff);

    println!("{:?}", &block);

    let h = block.hash();

    block.mine();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
    
    


}
