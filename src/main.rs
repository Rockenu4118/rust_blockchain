mod chain;
mod message;
mod network;

use crate::chain::account::Account;
use crate::chain::blockchain::Blockchain;
use crate::chain::transaction::Transaction;
use crate::network::client::Client;
use crate::network::server::Server;

fn chain_example() -> () {
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

fn main() -> std::io::Result<()> {
    // chain_example();
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("  {} server <addr:port>", args[0]);
        eprintln!("  {} client <addr:port>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "server" => {
            let server = Server::new("127.0.0.1:6000");
            server.run()?;
        }
        "client" => {
            let mut client = Client::connect(&args[2])?;

            let stdin = std::io::stdin();

            loop {
                let mut input = String::new();
                stdin.read_line(&mut input)?;
                client.send(&input);
            }
        }
        _ => println!("Unknown mode"),
    }

    Ok(())
}
