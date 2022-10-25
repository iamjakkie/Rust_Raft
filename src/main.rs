use std::{env, thread};
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod node;


fn main() {
    let peers: Vec<SocketAddr> = vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3330),
                                      SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3331)];
    let nodes: Vec<u16> = vec![3330, 3331];

    node::init(
        nodes[0],
        peers
            .iter()
            .map(|addr| *addr)
            .collect::<HashSet<SocketAddr>>()
    );

    println!("Finished");
}
