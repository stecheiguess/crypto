use blockchain::chain::Chain;

mod blockchain;
mod server;
mod transaction;

pub fn main() {
    let mut c = Chain::new();

    let db = ["test1", "test2", "whee", "wow"];

    for x in db.iter() {
        c.add(x)
    }

    //c.chain[2].data = "x".to_owned();

    match c.validate() {
        Ok(_) => println!("YAY"),
        Err(_) => println!("AWW"),
    }
}

// TODO - tests.
