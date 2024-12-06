// Declare the lab3 server

pub mod lab3;
mod macros;
use lab3::return_wrapper::ReturnWrapper;
use lab3::server::Server;
use std::env;

pub const ARG_COUNT: usize = 2;
pub const ADDR_INDEX: usize = 1;
pub const PROGRAM_NAME_INDEX: usize = 0;
pub const SUCCESS: u8 = 0;
pub const ARG_ERROR: u8 = 1;

/// Output the usage of the program
fn usage(program_name: &String) {
    stdout_writeln!("usage: ./{} [addr]", program_name);
}

/// Parse the arguments and save the configuration file name
fn parse_args(addr: &mut String) -> Result<(), u8> {
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

    Ok(())
}
fn main() -> ReturnWrapper {
    let mut addr: String = Default::default();
    match parse_args(&mut addr) {
        Ok(_) => (),
        Err(error_code) => {
            return ReturnWrapper::new(error_code);
        }
    }

    let mut server = Server::new();
    match server.open(&mut addr) {
        Ok(_) => (),
        Err(error_code) => {
            return ReturnWrapper::new(error_code);
        }
    }

    server.run();
    ReturnWrapper::new(SUCCESS)
}
