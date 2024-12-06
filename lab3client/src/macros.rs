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

#[macro_export]
macro_rules! mutex_lock_ref {
    ($mutex:expr) => {
        match $mutex.lock() {
            Ok(ref guard) => guard,
            Err(e) => panic!("Mutex poison error: {}", e),
        }
    };
}

#[macro_export]
macro_rules! mutex_lock_mut {
    ($mutex:expr) => {
        match $mutex.lock() {
            Ok(mut guard) => guard,
            Err(e) => panic!("Mutex poison error: {}", e),
        }
    };
}
