use std::env::Args;
use std::net::SocketAddr;

pub struct Config {
    pub server_addr: SocketAddr,
    pub data_filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let server_addr = match args.next() {
            Some(arg) => {
                match arg.parse() {
                    Ok(socket) => socket,
                    Err(_) => return Err("The Socket Address could not be parsed")
                }},
            None => return Err("No Socket Address was given"),
        };

        let data_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("There was no filename to store the recieved data in"),
        };

        Ok(Config { server_addr, data_filename })
    }
}
