use std::net::{IpAddr, SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use std::{io, thread};
use std::collections::HashSet;
use std::time::Duration;

pub async fn init(port:u16, peer_addrs: HashSet<SocketAddr>) {
    let peers = peer_addrs
        .iter()
        .map(|socket| socket.ip())
        .collect::<HashSet<IpAddr>>();

    println!("Spawning node");
    connect(Vec::from_iter(peer_addrs));

    listen(port).await;

    loop {

    }
}

// async fn process_socket<T>(socket: T) {
//     // do work with socket here
//
// }

pub async fn listen(port:u16){
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    println!("Server started");
    // log::info!("Server started");


    loop {
        if let Ok((socket, _)) = listener.accept().await {

            // process_socket(socket).await;
        }
    }
}

pub fn connect(
    mut peer_addrs: Vec<SocketAddr>
){
    // log::debug!("Connecting to peer...");
    println!("Connecting to peer...");

    // let mut streams = vec![];
    let mut retries :u8 = 0;

    for peer_addr in peer_addrs {
        task::spawn(async move {
            loop {
                match TcpStream::connect(peer_addr).await {
                    Ok(stream) => {
                        // send_handshake(peer_addr);
                        // log::debug!("Successfully connected with ({peer_addr})");
                        println!("Successfully connected with ({peer_addr})");
                        task::sleep(Duration::from_millis(5000)).await;
                        // streams.push(stream);
                    }
                    _ => {
                        // log::warn!("Connection failed ({peer_addr})");
                        println!("Connection failed ({peer_addr})");
                        task::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    //while let Some(peer_addr) = peer_addrs.pop() {
    // while let Some(peer_addr) = peer_addrs.first() {
    //     match TcpStream::connect(peer_addr) {
    //         Ok(stream) => {
    //             send_handshake(peer_addr);
    //             // log::debug!("Successfully connected with ({peer_addr})");
    //             println!("Successfully connected with ({peer_addr})");
    //             peer_addrs.remove(0);
    //             streams.push(stream);
    //         }
    //         _ => {
    //             // log::warn!("Connection failed ({peer_addr})");
    //             println!("Connection failed ({peer_addr})");
    //             thread::sleep(Duration::from_millis(5000));
    //         }
    //     }
    // }
    println!("Successfully connected with all peers");
    // log::info!("Successfully connected with all peers");
}