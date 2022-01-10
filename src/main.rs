#![warn(clippy::all, clippy::pedantic)]

use std::io;
use std::net::{TcpStream, Shutdown};
use std::vec::Vec;

fn main() {
    let mut buffer = String::new();
    let mut ports = Vec::new();

    println!("Give an IP-address to scan");
    io::stdin().read_line(&mut buffer).unwrap();

    buffer = buffer.trim_end_matches('\n').to_string();
    for port in 1..=65535 {
        let address = buffer.to_owned() + ":" + &port.to_string(); 
        if let Ok(stream) = TcpStream::connect(&address) {
            println!("[+] Port {} is open", port);
            stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
            ports.push(port);
        } else {
        }
    }
    println!("{:?}", ports);
}
