// main file of the lab3 test client
pub mod return_wrapper;
use crate::return_wrapper::ReturnWrapper;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
pub const ARG_COUNT: usize = 3;
pub const TOKEN_INDEX: usize = 2;
pub const ADDR_INDEX: usize = 1;
pub const PROGRAM_NAME_INDEX: usize = 0;
pub const SUCCESS: u8 = 0;
pub const ARG_ERROR: u8 = 1;
pub const READER_ERROR: u8 = 2;
pub const CONNECT_ERROR: u8 = 3;

/// Output the usage of the program
fn usage(program_name: &String) {
    eprintln!("usage: ./{} [addr]", program_name);
}

/// Parse the arguments and save the configuration file name
fn parse_args(addr: &mut String, token: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() != ARG_COUNT {
        usage(&args[PROGRAM_NAME_INDEX]);
        return Err(ARG_ERROR);
    }

    // Save config file name from args without destroying the original
    *addr = args[ADDR_INDEX].clone();
    *token = args[TOKEN_INDEX].clone();

    Ok(())
}
fn main() -> ReturnWrapper {
    let mut addr: String = Default::default();
    let mut token: String = Default::default();
    match parse_args(&mut addr, &mut token) {
        Ok(_) => (),
        Err(error_code) => {
            return ReturnWrapper::new(error_code);
        }
    }
    // adding newline as delimiter
    token = token + &"\n".to_string();
    // start client connection

    if let Ok(mut client_stream) = TcpStream::connect(&addr) {
        client_stream
            .write_all(token.as_bytes())
            .expect("writing to server failed");
        client_stream.flush().expect("flush failed");
        // waiting for response
        if token == "quit".to_string() {
            let duration = std::time::Duration::new(1, 0);
            std::thread::sleep(duration);
            // calling connect again to make sure the flag is loaded
            if let Err(_) = TcpStream::connect(&addr) {
                eprintln!("connection error");
                return ReturnWrapper::new(CONNECT_ERROR);
            }
        } else {
            println!("waiting for server response");
            let reader = BufReader::new(&client_stream);
            for line in reader.lines() {
                match line {
                    Ok(text) => {
                        println!("{}", text);
                    }
                    Err(_) => {
                        eprintln!("error while reading line");
                        return ReturnWrapper::new(READER_ERROR);
                    }
                }
            }
        }
        client_stream.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        eprintln!("connection error");
        return ReturnWrapper::new(CONNECT_ERROR);
    }
    ReturnWrapper::new(SUCCESS)
}
