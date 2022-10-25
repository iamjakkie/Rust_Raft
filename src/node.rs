use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::{io, thread};
use std::collections::HashSet;
use std::time::Duration;

pub fn init(port:u16, peer_addrs: HashSet<SocketAddr>) {
    let peers = peer_addrs
        .iter()
        .map(|socket| socket.ip())
        .collect::<HashSet<IpAddr>>();

    thread::spawn(move || {
        connect(Vec::from_iter(peer_addrs))
    });

    thread::spawn(move || {
        listen(port)
    });
}

pub fn listen(port:u16){
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();

    println!("Server started");
    // log::info!("Server started");

    for stream in listener.incoming() {
        //logic
    }
}

pub fn connect(
    mut peer_addrs: Vec<SocketAddr>
){
    // log::debug!("Connecting to peer...");
    println!("Connecting to peer...");

    let mut streams = vec![];

    while let Some(peer_addr) = peer_addrs.first() {
        match TcpStream::connect(peer_addr) {
            Ok(stream) => {
                // log::debug!("Successfully connected with ({peer_addr})");
                println!("Successfully connected with ({peer_addr})");
                peer_addrs.remove(0);
                streams.push(stream);
            }
            _ => {
                // log::warn!("Connection failed ({peer_addr})");
                println!("Connection failed ({peer_addr})");
                thread::sleep(Duration::from_millis(5000));
            }
        }
    }
    println!("Successfully connected with all peers");
    // log::info!("Successfully connected with all peers");
}