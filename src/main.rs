use std::net::{TcpStream};
use std::io::{Read, Write};
use std::env;
use moessbauer_client::Config;
use std::process;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let data_filepath = Path::new(&config.data_filename);
    let mut data_file = File::create(&data_filepath)?;

    match TcpStream::connect(config.server_addr) {
        Ok(mut stream) => {
            println!("Connected! to {}", config.server_addr);
            let msg = b"Hello, awaiting Data...";
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply");

            let mut data = [0 as u8; 120];
            loop {
                match stream.read(&mut data) {
                    Ok(chunk_len) => {
                        let mut pos = 0;
                        while pos < chunk_len {
                            let bytes_written = data_file.write(&data[pos..chunk_len])?;
                            pos += bytes_written;
                        }
                    },
                    Err(e) => {
                        eprintln!("Something went Wrong reading: {}", e);
                        process::exit(1);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated");
    Ok(())
}
