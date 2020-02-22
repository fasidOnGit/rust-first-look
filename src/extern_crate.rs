extern crate rand;
extern crate phrases;
use self::rand::Rng;
use self::phrases::greetings::french::{hello, goodbye};

pub fn crates() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();

    create_our_own_crates();
}

fn create_our_own_crates() {
    println!("English: {} {}",
             phrases::greetings::english::hello(),
             phrases::greetings::english::goodbye()
    );
    println!("French: {} {}",
             hello(),
             goodbye()
    )
}
