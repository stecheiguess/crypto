use std::time::Instant;

use chain::Chain;

pub mod block;
pub mod chain;

pub fn trial() {
    let mut c = Chain::new();

    let mut avg = 0;

    for i in 1..=1000 {
        let x = format!("Block {}", i);

        let now = Instant::now();
        c.add(x.as_str());
        let elapsed = now.elapsed().as_millis();

        avg = (avg * (i - 1) + elapsed) / i;

        println!("Mined {}: taken {}ms. AVG: {}ms.", x, elapsed, avg)
    }

    //c.chain[2].data = "x".to_owned();

    match c.validate() {
        Ok(_) => println!("YAY"),
        Err(_) => println!("AWW"),
    }
}

// TODO - tests.
