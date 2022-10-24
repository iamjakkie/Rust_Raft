use std::net::TcpListener;

pub fn start(num:i8) {
    let addr = ["0.0.0.0:333", &num.to_string()].join("");
    let listener = TcpListener::bind(&addr).unwrap();
    println!("New node started: {}", &addr);

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}