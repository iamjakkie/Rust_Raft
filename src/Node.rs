use std::net::TcpListener;

pub fn start(num:i8) {
    let addr = ["0.0.0.", &num.to_string(), ":3334"].join("");
    let listener = TcpListener::bind(&addr).unwrap();
    println!("New node started: {}", &addr);
}