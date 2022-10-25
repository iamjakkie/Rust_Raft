use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("0.0.0.0:3330").expect("Could not connect");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        io::stdin().read_line(&mut input).expect("Failed to read stdin");
        stream.write(input.as_bytes()).expect("Failed to write to stream");

    }
}