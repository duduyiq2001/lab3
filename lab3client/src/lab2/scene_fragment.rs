use crate::lab2::declarations::*;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::JoinHandle;
use crate::lab2::script_gen::grab_trimmed_file_lines;
use crate::{ stderr_writeln, stdout_writeln };
use crate::mutex_lock_ref;
use crate::mutex_lock_mut;
use std::sync::{ Arc, Mutex, MutexGuard };
// PlayConfig: Vec[(name of character, file name), ...]
type PlayConfig = Vec<(String, String)>;

pub const TITLE_INDEX: usize = 0;
pub const FIRST_LINE_INDEX: usize = 1;
pub const CHARACTER_NAME_INDEX: usize = 0;
pub const FILE_NAME_INDEX: usize = 1;
pub const TOKEN_NUMBER: usize = 2;

/// Minimum number of lines in the config file
/// PS: Not on the instructions, Avoid using hardcoded numbers
pub const MIN_CONFIG_LINES: usize = 2;

pub(crate) struct SceneFragment {
    title: String,
    players: Vec<Arc<Mutex<Player>>>,
}

impl SceneFragment {
    pub fn new(title: &String) -> Self {
        Self {
            title: (*title).clone(),
            players: Vec::<Arc<Mutex<Player>>>::new(),
        }
    }

    pub fn contain_name(&self, name: &str) -> bool {
        for player in &self.players {
            if name == mutex_lock_ref!(player).player_name {
                return true;
            }
        }
        false
    }

    pub fn has_title(&self) -> bool {
        return self.title != "";
    }
    /// Parse the `config_line` to type `SceneFragmentConfig` and push it to `SceneFragment_configs`
    pub fn add_config(config_line: &String, scene_frag_config: &mut PlayConfig) {
        let tokens: Vec<&str> = config_line.split_whitespace().collect();

        if tokens.len() != TOKEN_NUMBER {
            // Invalid tokens number for this config_line
            // Just complain, get the previous two tokens
            if SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                stderr_writeln!(
                    "WHINGE: INVALID CONFIG LINE WITH {} TOKENS, LINE: \n{}\n",
                    tokens.len(),
                    config_line
                );
            }
        }

        if tokens.len() >= TOKEN_NUMBER {
            // config_line seems to be valid
            // Get previous character_name and file_name even if there are extra tokens
            let character_name = tokens[CHARACTER_NAME_INDEX].to_string();
            let file_name = tokens[FILE_NAME_INDEX].to_string();
            scene_frag_config.push((character_name, file_name));
        }
    }

    /// Read the `config_file_name` file and parse it to `title` and `SceneFragment_config`
    pub fn read_config(
        config_file_name: &String,
        scene_frag_config: &mut PlayConfig
    ) -> Result<(), u8> {
        let mut config_lines: Vec<String> = Vec::new();

        // Read the config file to lines
        grab_trimmed_file_lines(config_file_name, &mut config_lines)?;
        // Will return Err(ERR_SCRIPT_GENERATION_FAIL) if the file can not be read

        for line in &config_lines {
            SceneFragment::add_config(line, scene_frag_config);
        }

        Ok(())
    }

    /// find the next player containing the `line_num`
    /// return the only player's index who has `line_num`
    pub fn find_next_player(self: &mut Self, line_num: usize) -> Option<usize> {
        let mut player_index: Option<usize> = None;
        for index in 0..self.players.len() {
            if let Some(next_l) = mutex_lock_ref!(self.players[index]).next_line() {
                if line_num == next_l {
                    player_index = Some(index);
                    break;
                }
            }
        }
        player_index
    }

    // report missing and duplicates right here
    pub fn recite(self: &mut SceneFragment) {
        // storing current character to keep track of chracater change
        if self.players.len() == 0 {
            return;
        }
        let mut current_character: String = String::new();
        // start reciting

        // get MAX_LINE_NUMBER
        let mut max_line_number: usize = 0;
        for player in &self.players {
            match player.lock() {
                Ok(player_ref) => {
                    let (player_max_line, _string) =
                        &player_ref.play_lines[player_ref.play_lines.len() - 1];
                    max_line_number = std::cmp::max(max_line_number, *player_max_line);
                }
                Err(e) => {
                    panic!("Mutex poison error: {}", e);
                }
            }
        }

        for line_to_speak in 0usize..max_line_number + 1 {
            let player_to_speak_index = self.find_next_player(line_to_speak);
            if None == player_to_speak_index {
                // missing line
                if SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                    stderr_writeln!("WHINGE: MISSING AT LINE NUMBER {}", line_to_speak);
                }
                continue;
            }
            mutex_lock_mut!(self.players[player_to_speak_index.unwrap()]).speak(
                &mut current_character
            );
            while let Some(player_index) = self.find_next_player(line_to_speak) {
                if None == player_to_speak_index {
                    break;
                }
                if SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                    stderr_writeln!(
                        "WHINGE: DUPLICATED AT LINE NUMBER {} AT PLAYER {}",
                        line_to_speak,
                        player_index
                    );
                }
                mutex_lock_mut!(self.players[player_to_speak_index.unwrap()]).speak(
                    &mut current_character
                );
            }
        }
    }
    pub fn process_config(self: &mut Self, config: &PlayConfig) {
        // all thread handles
        let mut handles: Vec<JoinHandle<Player>> = Vec::<JoinHandle<Player>>::new();
        // ScriptConfig: Vec[(name of character, file name), ...]
        for element in config {
            match element {
                (character_name, part_file) => {
                    let mut player = Player::new(character_name);
                    let p_file = part_file.clone();
                    let handle = thread::spawn(move || -> Player {
                        player.prepare(&p_file);
                        return player;
                    });
                    handles.push(handle);
                }
            }
        }
        // handles is Vec<JoinHandle<SceneFragment>>
        let mut idx = 0;
        for handle in handles {
            match handle.join() {
                Ok(player) => {
                    self.players.push(Arc::new(Mutex::new(player)));
                    idx += 1;
                }
                Err(_) => {
                    // repanics if thread fails
                    panic!("error in thread {}", idx);
                }
            }
        }
    }

    /// Get the `title` and `SceneFragment` from the `config_file_name` file
    pub fn prepare(self: &mut Self, config_file_name: &String) {
        let mut scene_frag_config: PlayConfig = PlayConfig::new();

        // Pass error code to the caller
        Self::read_config(config_file_name, &mut scene_frag_config).unwrap();
        Self::process_config(self, &scene_frag_config);
        // Sorting players based on first line num
        self.players.sort_by(|x, y| { Self::compare_player(x, y) });
    }
    /// test if the string is not blank
    pub fn non_blank(line: &String) -> bool {
        if line.len() == 0 {
            return false;
        }

        for ch in line.chars() {
            if ch != ' ' {
                return true;
            }
        }

        false
    }

    /// print out messages listing the entrances of all players that were not in the other (previous)
    /// scene fragment, in the order in which they first
    /// speak in the current scene (i.e., in the order in which they are sorted in the SceneFragment),
    pub fn enter(self: &Self, other: &Self) {
        if Self::non_blank(&self.title) {
            stdout_writeln!("{}", &self.title);
        }
        for player in &self.players {
            if !other.contain_name(&mutex_lock_ref!(player).player_name) {
                stdout_writeln!("[Enter {}.]", mutex_lock_ref!(player).player_name);
            }
        }
    }

    /// prints out messages listing the entrances of all players in the scene (again in order).
    pub fn enter_all(self: &Self) {
        if Self::non_blank(&self.title) {
            stdout_writeln!("{}", &self.title);
        }
        for player in &self.players {
            stdout_writeln!("[Enter {}.]", mutex_lock_ref!(player).player_name);
        }
    }

    /// print out messages listing the exits of all players that will not be in the other (subsequent)
    /// scene fragment, in the reverse of the order in which they first speak in the current scene (
    pub fn exit(self: &Self, other: &Self) {
        for player in self.players.iter().rev() {
            if !other.contain_name(&mutex_lock_ref!(player).player_name) {
                stdout_writeln!("[Exit {}.]", mutex_lock_ref!(player).player_name);
            }
        }
    }

    /// prints out messages listing the entrances of all players in the scene (again in order).
    pub fn exit_all(self: &Self) {
        for player in self.players.iter().rev() {
            stdout_writeln!("[Exit {}.]", mutex_lock_ref!(player).player_name);
        }
    }

    pub fn compare_player(
        player_a: &Arc<Mutex<Player>>,
        player_b: &Arc<Mutex<Player>>
    ) -> std::cmp::Ordering {
        // getting atomic references
        let a_ref: &MutexGuard<'_, Player>;
        let b_ref: &MutexGuard<'_, Player>;
        let binding_a = (*player_a).lock();
        let binding_b = (*player_b).lock();
        match binding_a {
            Ok(ref a) => {
                a_ref = a;
            }
            Err(_e) => {
                return std::cmp::Ordering::Equal;
            }
        }

        match binding_b {
            Ok(ref b) => {
                b_ref = b;
            }
            Err(_e) => {
                return std::cmp::Ordering::Equal;
            }
        }

        // comparison

        match Player::partial_cmp(a_ref, b_ref) {
            Some(ord) => {
                return ord;
            }
            None => {
                return std::cmp::Ordering::Equal;
            }
        }
    }
}
