use crate::lab2::declarations::SHOULD_COMPLAIN;
use std::sync::atomic::Ordering as AtOrd;
use std::cmp::Ordering;
use crate::lab2::declarations::Player;
use crate::{ stderr_writeln, stdout_writeln };

const FIRST_LINE_INDEX: usize = 0;
impl Player {
    pub fn new(player_name: &String) -> Self {
        Self {
            player_name: (*player_name).clone(),
            play_lines: Vec::new(),
            current_index: 0,
        }
    }

    /// ## Parse a single `unparsed_line` into a tuple with `character` and push them to `play`
    /// * unparsed_line: "<line_number> <line_text>"
    pub fn add_script_line(self: &mut Player, unparsed_line: &String) {
        if unparsed_line.len() > 0 {
            // Now, unparsed_line is not empty

            // Parse the line into a tuple (line_number, line_text)
            // etc.
            //      "32 Hello, World!"
            if
                let Some((line_number_str, line_text)) = unparsed_line.split_once(
                    char::is_whitespace
                )
            {
                let line_number_str = line_number_str.trim();
                let line_text = line_text.trim();

                // Parse line_number_str to usize
                match line_number_str.parse::<usize>() {
                    Ok(line_number) => {
                        // Success
                        // Push the tuple to the play vector
                        self.play_lines.push((line_number, line_text.to_string()));
                    }
                    Err(_) => {
                        // Invalid line number, Just skip this line
                        if SHOULD_COMPLAIN.load(AtOrd::SeqCst) {
                            stderr_writeln!(
                                "WHINGE: INVALID LINE NUMBER IN CHARACTER{}, LINE:\n{}\n",
                                self.player_name,
                                unparsed_line
                            );
                        }
                    }
                }
            }
        }
    }

    /// ## Read the `part_file` and parse it into `Player`
    /// * part_file: &String, the path of script for this player
    /// need to capture open file error
    pub fn prepare(self: &mut Player, part_file: &String) {
        // NOT OK TO USE UNWRAP HERE, IF THE FILE DOES NOT EXIST THEN WE SHOULD HANDLE IT WITH SCRIPT_GEN_ERROR
        if let Ok(part_file_content) = std::fs::read_to_string(part_file) {
            // Split the part file content by lines
            let lines = part_file_content.split('\n').collect::<Vec<&str>>();

            for line in lines {
                // Just read all the unparsed_lines into the file
                self.add_script_line(&line.to_string());
                // TODO: may raise error
            }

            // Deal with the unordered lines
            self.play_lines.sort_by_key(|k| k.0);
        } else {
            panic!("file {} can not be opened!", part_file);
        }
    }
    /// Print the current script line of the current player, and
    /// switch the current player if the owner of the current
    /// script line and the current player not match
    pub fn speak(self: &mut Player, recently_player: &mut String) {
        // TODO: Implement this method

        if self.current_index >= self.play_lines.len() {
            // Simply return
            return;
        }

        // updating recent player
        if *recently_player != self.player_name {
            // Change player here
            *recently_player = self.player_name.clone();

            stdout_writeln!("\n{}.", self.player_name);
        }

        // print current script line
        let (_, line_content) = self.play_lines[self.current_index].clone();
        stdout_writeln!("{}", line_content);

        // Increase the index
        self.current_index += 1;
    }

    /// return the valid script line number of the player,
    /// return None if the line number is invalid
    pub fn next_line(self: &Player) -> Option<usize> {
        if self.current_index < self.play_lines.len() {
            let (line_number, _line_content) = self.play_lines[self.current_index].clone();
            return Some(line_number);
        } else {
            None
        }
    }
}

/// Two players are equal if neither of them has any lines to speak (both are silent characters) or if they
/// both have lines to speak and have the same first
/// line number (as indicated by the line number value in the tuple at the 0th position in their vector of lines to speak)
/// pub(crate) play_lines: Vec<(usize, String), Global
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match (self.play_lines.first(), other.play_lines.first()) {
            (None, None) => true, // Both are silent characters
            (Some((self_index_num, _)), Some((other_index_num, _))) =>
                self_index_num == other_index_num,
            _ => false, // One has lines while the other does not
        }
    }
}

/// implment full equal for player
impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A player is strictly less than another player (and the other player is thus strictly greater)
/// if (1) they have no lines to speak and the other
/// player does, or (2) both have lines to speak and they have a lower first line number than the other player.
impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.play_lines.len() {
            0 => { self.play_lines.len().cmp(&other.play_lines.len()) }
            _ => {
                match other.play_lines.len() {
                    0 => { Ordering::Greater }
                    _ => {
                        match &self.play_lines[FIRST_LINE_INDEX] {
                            (index_num, _line) => {
                                match &other.play_lines[FIRST_LINE_INDEX] {
                                    (num, _line) => { index_num.cmp(num) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
