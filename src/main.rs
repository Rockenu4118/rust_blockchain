mod account;
mod block;
mod blockchain;
mod transaction;

use crate::account::Account;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

fn main() {
    // let block = Block::new(0, [0; 32], [0; 32], 0);

    // println!("{:?}", block); // Debug print
    // println!("{:#?}", block); // Pretty-printed Debug
    // println!("{}", hex::encode(0x00));

    let mut blockchain = Blockchain::new(4);
    let account1 = Account::new(String::from("aj"));
    let account2 = Account::new(String::from("justin"));

    let tx1 = Transaction::new(
        account1.public_key.clone(),
        account2.public_key.clone(),
        100,
    );

    print!("{}", account1);
    print!("{}", account2);
    println!("");

    print!("{}", blockchain.tip());

    blockchain.add_transaction(tx1);

    println!("Mining...");
    blockchain.mine_block();
    print!("{}", blockchain.tip());

    println!("AJ Balance: {}", blockchain.balances[&account1.public_key]);

    println!("Mining...");
    blockchain.mine_block();
    print!("{}", blockchain.tip());
}
