// Implement the server module
use std::io::Write;
use std::io::{BufReader, Read};
use std::thread::JoinHandle;
use std::{net::TcpListener, net::TcpStream, sync::atomic::AtomicBool};
static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);
use crate::{stderr_writeln, stdout_writeln};
use std::thread;

use std::fs::File;
use std::io::prelude::*;
pub struct Server {
    listener: Option<TcpListener>,
    listening_addr: String,
}

pub const BINDING_ERROR: u8 = 1;
pub const FILE_ERROR: u8 = 2;
pub const FIRST_TOKEN_INDEX: usize = 0;
impl Server {
    pub fn new() -> Self {
        Server {
            listener: None,
            listening_addr: "".to_string(),
        }
    }

    pub fn is_open(&self) -> bool {
        return !self.listener.is_none();
    }

    pub fn open(&mut self, addr: &str) -> Result<(), u8> {
        if let Ok(listener) = TcpListener::bind(addr) {
            self.listener = Some(listener);
            self.listening_addr = addr.to_string();
            Ok(())
        } else {
            stderr_writeln!("binding failed with {}", addr);
            Err(BINDING_ERROR)
        }
    }

    pub fn run(&mut self) {
        stdout_writeln!("listening on {}", &self.listening_addr);
        // storing thread handles
        let mut handles: Vec<JoinHandle<()>> = Vec::<JoinHandle<()>>::new();
        if let Some(listener) = &self.listener {
            loop {
                let (stream, addr) = listener.accept().unwrap();
                stdout_writeln!("established connection with client {addr}");
                let handle = thread::spawn(move || {
                    handle_client(stream);
                });
                //TRACKING HANDLE
                handles.push(handle);

                if CANCEL_FLAG.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }
        } else {
            stderr_writeln!("listener not set up, please run Server::open");
        }

        // joining threads
        // handles is Vec<JoinHandle<SceneFragment>>
        for handle in handles {
            if let Err(e) = handle.join() {
                stderr_writeln!("error occurred in server thread :{:?}", e);
            }
        }
    }
}
///
/// making sure user can only access files at
/// current directory
fn is_safe_filename(filename: &str) -> bool {
    !filename.contains(|c| match c {
        '/' | '\\' | '$' => true,
        _ => false,
    }) && !filename.contains("..")
}

fn process_file(arg: &str) -> Result<Vec<u8>, u8> {
    // sanity check file names for security
    if !is_safe_filename(arg) {
        return Err(FILE_ERROR);
    }

    if let Ok(file) = File::open(&arg) {
        let mut reader = BufReader::new(file);
        let mut buf = Vec::<u8>::new();
        match reader.read_to_end(&mut buf) {
            Ok(_) => {
                return Ok(buf);
            }
            Err(_) => {
                stderr_writeln!("file could not be read!");
                Err(FILE_ERROR)
            }
        }
    } else {
        stderr_writeln!("file could not be opened!");

        Err(FILE_ERROR)
    }
}

pub fn handle_client(mut stream: TcpStream) {
    // copy stream before being consumed by buf reader

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    let _ = reader
        .read_line(&mut line)
        .expect("failed to read from stream");
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 0 {
        // skip blank lines
        stderr_writeln!("no input from user");
        return;
    }

    match tokens[FIRST_TOKEN_INDEX] {
        "quit" => {
            CANCEL_FLAG.store(true, std::sync::atomic::Ordering::SeqCst);
            return;
        }
        // treat the token as file and try to open that file
        _ => {
            if let Ok(content) = process_file(tokens[FIRST_TOKEN_INDEX]) {
                stream.write(&content).unwrap();
            } else {
                stderr_writeln!("no such file or file unauthorized!");
                return;
            }
        }
    }
}
