// define macros
#[macro_export]
macro_rules! stdout_writeln {
    // ... macro definition ...
    ($($arg:tt)*) => {
        use std::io::Write;
        match writeln!(std::io::stdout().lock(), $($arg)*) {
            Ok(_) => (),
            Err(e) => {
                panic!("Failed to write to stderr: {}", e);
            }
        }
    };
}

#[macro_export]
macro_rules! stderr_writeln {
    // ... macro definition ...
    ($($arg:tt)*) => {
        use std::io::Write;
        match writeln!(std::io::stderr().lock(), $($arg)*) {
            Ok(_) => (),
            Err(e) => {
                panic!("Failed to write to stderr: {}", e);
            }
        }
    };
}
