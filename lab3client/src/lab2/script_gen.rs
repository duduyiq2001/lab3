use std::fs::File;
use std::io::{ BufRead, BufReader, Read };
use std::net::TcpStream;
use crate::lab2::declarations;
use crate::stderr_writeln;
use std::io::Write;

use super::declarations::IO_ERROR;

/// Read the `file_name` file and push the trimmed lines to the vector `lines`
/// PS: Can read any file, not just the config file
pub fn grab_trimmed_file_lines(file_name: &str, lines: &mut Vec<String>) -> Result<(), u8> {
    let file_result = get_buffered_reader(&file_name.to_string());
    if let Err(_) = file_result {
        stderr_writeln!("ERROR: CAN NOT OPEN FILE {}", file_name);
        return Err(declarations::ERR_SCRIPT_GENERATION_FAIL);
    }
    let mut reader = file_result?;
    let mut line_content = String::new();

    loop {
        line_content.clear();

        if let Err(_) = reader.read_line(&mut line_content) {
            // Read line failed
            stderr_writeln!("ERROR: CAN NOT READ LINE FROM FILE {}", file_name);
            return Err(declarations::ERR_SCRIPT_GENERATION_FAIL);
        }

        if line_content.is_empty() {
            // End of file
            break;
        }

        let trimmed_line = line_content.trim().to_string();

        if !trimmed_line.is_empty() {
            // Deal with non-empty line only
            lines.push(trimmed_line);
        } else {
            // Just skip
        }
    }
    return Ok(());

    // let file_result = File::open(file_name);
    // match file_result {
    //     Ok(file) => {
    //         let mut reader = BufReader::new(file);
    //         let mut line_content = String::new();

    //         loop {
    //             line_content.clear();

    //             if let Err(_) = reader.read_line(&mut line_content) {
    //                 // Read line failed
    //                 stderr_writeln!("ERROR: CAN NOT READ LINE FROM FILE {}", file_name);
    //                 return Err(declarations::ERR_SCRIPT_GENERATION_FAIL);
    //             }

    //             if line_content.is_empty() {
    //                 // End of file
    //                 break;
    //             }

    //             let trimmed_line = line_content.trim().to_string();

    //             if !trimmed_line.is_empty() {
    //                 // Deal with non-empty line only
    //                 lines.push(trimmed_line);
    //             } else {
    //                 // Just skip
    //             }
    //         }
    //         Ok(())
    //     }
    //     Err(_) => {
    //         // Open file failed
    //         stderr_writeln!("ERROR: CAN NOT OPEN FILE {}", file_name);
    //         Err(declarations::ERR_SCRIPT_GENERATION_FAIL)
    //     }
    // }
}

pub fn is_valid_net_str(s: &str) -> bool {
    // Check prefix
    if !s.starts_with("net:") {
        return false;
    }

    // Split the rest by colons
    let parts: Vec<&str> = s[4..].split(':').collect();
    if parts.len() != 3 {
        return false;
    }

    // Check IP address format
    let ip_parts: Vec<&str> = parts[0].split('.').collect();
    if ip_parts.len() != 4 {
        return false;
    }

    // Validate each IP octet is a valid number 0-255
    for part in ip_parts {
        if let Ok(_num) = part.parse::<u8>() {
            // Valid number 0-255
        } else {
            return false;
        }
    }

    // Check port is valid number
    if let Ok(_port) = parts[1].parse::<u16>() {
        // Valid port number
    } else {
        return false;
    }

    // Filename must not be empty
    if parts[2].is_empty() {
        return false;
    }

    true
}

pub fn get_buffered_reader(info: &String) -> Result<BufReader<Box<dyn Read>>, u8> {
    if is_valid_net_str("net:") {
        let tokens: Vec<&str> = info.split(':').collect();
        if let [_, addr, port, f, ..] = tokens[..] {
            let mut file_name = f.to_string();
            // adding \n for delimiter
            file_name = file_name + "\n";

            if let Ok(mut stream) = TcpStream::connect(addr.to_string() + ":" + port) {
                //sending the file name
                if let Err(_) = stream.write(file_name.as_bytes()) {
                    stderr_writeln!("failed to write to stream!");
                    return Err(IO_ERROR);
                }

                return Ok(BufReader::new(Box::new(stream)));
            } else {
                stderr_writeln!("failed to connect to address: {}", addr);
                return Err(IO_ERROR);
            }
        } else {
            stderr_writeln!("token string invalid");
            return Err(IO_ERROR);
        }
    } else {
        // treating it as a file
        if let Ok(file) = File::open(info) {
            return Ok(BufReader::new(Box::new(file)));
        } else {
            stderr_writeln!("failed to open file {}", info);
            return Err(IO_ERROR);
        }
    }
}
