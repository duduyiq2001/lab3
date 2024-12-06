// Implement the return wrapper

use std::process::{ExitCode, Termination};

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
        if self.value != crate::SUCCESS {
            eprintln!("ERROR CODE: {}", self.value);
        }
        ExitCode::from(self.value)
    }
}
