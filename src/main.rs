use std::{env, thread};

mod Node;



fn main() {

    let arg = env::args().nth(1).unwrap();
    let ind: i8 = arg.parse().unwrap();

    println!("{}", ind);

    Node::start(ind);

    println!("Finished");
}
