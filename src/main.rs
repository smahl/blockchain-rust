use blockchainlib::*;
fn main () {
    let mut block = Block::new(13,12345,vec![0; 32], 1, "Genesis block!".to_owned());

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
    
    


}
