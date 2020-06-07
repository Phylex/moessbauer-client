use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    let adress = "127.0.0.1:3333";
    match TcpStream::connect(adress) {
        Ok(mut stream) => {
            println!("Connected! to {}", adress);
            let msg = b"Hello";
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply");

            let mut data = [0 as u8; 5];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated");
}
