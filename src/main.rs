extern crate clap;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::env;
use std::process;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use clap::{Arg, App, SubCommand};
use moessbauer_client::Config;


fn main() -> Result<(), std::io::Error> {

    // read the command line arguments
    let matches = App::new("Moessbauer Measurement Client")
        .version("0.1")
        .author("Alexander Becker <nabla.becker@mailbox.org")
        .arg(Arg::with_name("interactive")
            .short("i"))
        .arg(Arg::with_name("m-value")
            .short("m")
            .long("mvalue")
            .value_name("m")
            .help("Multiplication factor of the filter.\n\
                  This factor is the decay constant\n\
                  of the observed pulse in ADC clock cycles")
            .next_line_help(true)
            .takes_value(true)
            .required_unless("interactive"))
        .arg(Arg::with_name("k-value")
            .short("k")
            .long("kvalue")
            .value_name("k")
            .help("Time in ADC clock cycles that the trapezoidal\n\
                   ramp takes to go from no signal to full signal\n\
                   height")
            .next_line_help(true)
            .takes_value(true)
            .required_unless("interactive"))
        .arg(Arg::with_name("l-value")
            .short("l")
            .long("lvalue")
            .help("Time of the plateau of the trapezoid\n\
                   in units of ADC clock cycles")
            .next_line_help(true)
            .takes_value(true)
            .required_unless("interactive"))
        .arg(Arg::with_name("peak_threshhold")
            .short("p")
            .long("pthresh")
            .help("minimum height of the trapezoid to be accepted\n\
                   as a signal by the filter. This has to be as low\n\
                   as possible but high enough to suppress noise that\n\
                   otherwise would overwhelm the system.")
            .next_line_help(true)
            .takes_value(true))
            .required_unless("interactive")
        .arg(Arg::with_name("totzeittime")
            .short("td")
            .long("totzeit")
            .help("minimum time between two events, this\n\
                   again is a way to reduce unwanted noise\n\
                   it is best to play around with this parameter\n\
                   to see what works best")
            .next_line_help(true)
            .takes_value(true)
            .required_unless("interactive"))
        .get_matches();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // create the file to write the data to
    let data_filepath = Path::new(&config.data_filename);
    let mut data_file = File::create(&data_filepath)?;

    // establish the connection with the instrument
    if let stream = TcpStream::connect(config.server_addr) {
        println!("Connection established");
    } else {
        println!("could not establish connection.\n Shutting down!");
        process::exit(1);
    }

    let (cmd_tx, cmd_rx) = mpsc::channel();
    // set up the cli_thread
    if config.interactive {
        let cmd_handle = thread::spawn(move || {
            moessbauer_client::cli::cliif(cmd_tx)
        });
    } else {
    }

    //set up the network io thread
    let (net_tx, net_rx) = mpsc::channel();
    let net_handle = thread::spawn(move || {
        moessbauer_client::netio::netctl(stream, net_tx, cmd_rx)
    });

    println!("Terminated");
    Ok(())
}
