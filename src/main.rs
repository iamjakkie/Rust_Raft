use std::thread;

mod Node;



fn main() {
    let nodes = 3;
    let handle = thread::spawn(move || {
        for i in 0..nodes {
            Node::start(i);
        }
    });

    handle.join().unwrap();

    println!("Finished");
}
