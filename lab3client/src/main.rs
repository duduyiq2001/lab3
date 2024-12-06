pub mod lab2;

mod macros;
use std::env;

use lab2::declarations;
use crate::lab2::play::Play;
use std::sync::atomic::Ordering;

use crate::lab2::return_wrapper::ReturnWrapper;

// Constants

// Length range of the arguments
const MIN_ARGS: usize = 2;
const MAX_ARGS: usize = 3;

// Position of the arguments
const PROGRAM_NAME_INDEX: usize = 0;
const CONFIG_FILE_NAME_INDEX: usize = 1;
const OPTIONAL_ARG_INDEX: usize = 2;

/// Output the usage of the program
fn usage(program_name: &String) {
    stdout_writeln!("usage: ./{} <script_file_name> [whinge]", program_name);
}

/// Parse the arguments and save the configuration file name
fn parse_args(config_file_name: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if
        args.len() < MIN_ARGS ||
        args.len() > MAX_ARGS ||
        (args.len() == MAX_ARGS && args[OPTIONAL_ARG_INDEX] != "whinge")
    {
        usage(&args[PROGRAM_NAME_INDEX]);
        return Err(declarations::ERR_BAD_COMMAND);
    }

    // Save config file name from args without destroying the original
    *config_file_name = args[CONFIG_FILE_NAME_INDEX].clone();

    // Optional argument exists
    if args.len() == MAX_ARGS {
        // args[INDEX_OPTIONAL_ARG] == "whinge"
        declarations::SHOULD_COMPLAIN.store(true, Ordering::SeqCst);
    }

    Ok(())
}

fn main() -> ReturnWrapper {
    // Config file name
    let mut script_file = String::new();

    match parse_args(&mut script_file) {
        Ok(_) => (),
        Err(error_code) => {
            return ReturnWrapper::new(error_code);
        }
    }

    // Creates a new SceneFragment struct
    let mut play = Play::new();

    // Call the prepare method and pass the config file name
    match play.prepare(&script_file) {
        Ok(()) => {
            play.recite();
        }
        Err(error_code) => {
            return ReturnWrapper::new(error_code);
        }
    }

    ReturnWrapper::new(declarations::SUCCESS)
}
