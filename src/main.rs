use std::io;
mod block;
mod blockchain;
mod hash;


use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Block 1".to_owned());
    blockchain.add_block("Block 2".to_owned());
    while true {
        println!("\n1. To Create A Block.\n 2. To View The Blockchain  \n Press Other Key to Exit \n");
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input);
        
        let input = input.trim();

        match input {
            "1" => {
                println!("Enter your Data : \n");
                let mut data = String::new();
                io::stdin()
                .read_line(&mut data)
                .expect("Failed to read input");

                blockchain.add_block(data.to_owned());
            }
            "2" => {
                println!("\n\n");
                for block in &blockchain.chain {
                    println!("Index: {}", block.index);
                    println!("Timestamp: {}", block.timestamp);
                    println!("Data: {}", block.data);
                    println!("Previous Hash: {}", block.previous_hash);
                    println!("Hash: {}", block.hash);
                    println!("-------------------");
                }
            }
            _ => {
                break;
            }
        }
        
    }

}
