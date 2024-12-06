// Lab2 Step 9
use crate::stderr_writeln;
use std::process::{ ExitCode, Termination };

use crate::lab2::declarations;

pub struct ReturnWrapper {
    value: u8,
}

impl ReturnWrapper {
    pub fn new(value: u8) -> Self {
        ReturnWrapper { value }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        // SUCCESS code
        if self.value != declarations::SUCCESS {
            stderr_writeln!("ERROR CODE: {}", self.value);
        }
        ExitCode::from(self.value)
    }
}
