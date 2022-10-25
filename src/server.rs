use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{env, thread};

fn handle_incoming(mut stream: TcpStream) {
    let mut data = [0 as u8, 100]; // 100b buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("Error when handling {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn start(num:i8) {
    let heartbeat = 5;
    let addr = ["0.0.0.0:333", &num.to_string()].join("");
    let listener = TcpListener::bind(&addr).unwrap();
    // listener.set_nonblocking(true).expect("set nonblocking");
    println!("New node started: {}", &addr);
    //
    // let mut to_connect: Vec<String> = vec![];
    //
    // if num > 0 {
    //     let mut port = num-1;
    //     while port >= 0 {
    //         let peer_addr = ["0.0.0.0:333", &port.to_string()].join("");
    //         println!("Added new node: {}", &peer_addr);
    //         to_connect.push(peer_addr);
    //         port-=1;
    //     }
    // }

    let mut connected: Vec<SocketAddr> = vec![];

    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {addr:?}");
                handle_incoming(_socket);
            },
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }

    // for stream in listener.incoming(){
    //     match stream {
    //         Ok(stream) => {
    //             thread::spawn(move || {
    //                 let peer_addr = stream.peer_addr().unwrap();
    //                 println!("New connection: {}", &peer_addr);
    //                 // connected.push(peer_addr); // implement Copy trait
    //                 handle_incoming(stream);
    //                 match TcpStream::connect(&peer_addr) {
    //                     Ok(mut s_stream) => {
    //                         println!("Successfully connected to: {}", &peer_addr);
    //                         let msg = b"test";
    //                         s_stream.write(msg).unwrap();
    //                     },
    //                     Err(e) => {
    //                         println!("Failed to connect to: {}", &peer_addr);
    //                     }
    //                 }
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //         }
    //     };
    // }
    // drop(listener);
}


fn main() {

    let arg = env::args().nth(1).unwrap();
    let ind: i8 = arg.parse().unwrap();

    println!("{}", ind);

    start(ind);

    println!("Finished");
}