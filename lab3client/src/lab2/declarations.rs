use std::sync::atomic::AtomicBool;

// pub type Play = Vec<(usize, String, String)>;

// Return codes
pub const SUCCESS: u8 = 0;
pub const ERR_BAD_COMMAND: u8 = 1;

// If the program should complain
pub static SHOULD_COMPLAIN: AtomicBool = AtomicBool::new(false);

/// indicating the error stems from script generation, while the command format is correct
pub const ERR_SCRIPT_GENERATION_FAIL: u8 = 2;
/// indicating that the error that happens before any config scripts were obtained
/// mostly IO/network/read/write error, we call it IO_ERROR
pub const IO_ERROR: u8 = 3;

/// Vec of (line number, line content)
type PlayLines = Vec<(usize, String)>;

pub struct Player {
    pub(crate) player_name: String,
    pub(crate) play_lines: PlayLines,
    pub(crate) current_index: usize,
}
