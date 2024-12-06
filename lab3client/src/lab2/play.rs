use crate::lab2::declarations::*;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::JoinHandle;
use crate::lab2::script_gen::grab_trimmed_file_lines;
use super::scene_fragment::SceneFragment;
use crate::stderr_writeln;
use crate::mutex_lock_ref;
use crate::mutex_lock_mut;
use std::sync::{ Arc, Mutex };

/// whether the text field represents the title of a new scene (true) or the name of a configuration file (false)
type ScriptConfig = Vec<(bool, String)>;

type Fragments = Vec<Arc<Mutex<SceneFragment>>>;
pub const TITLE_INDEX: usize = 0;
pub const FIRST_TOKEN_INDEX: usize = 0;
pub const SCENE_TITLE_INDEX: usize = 0;
pub const FIRST_LINE_INDEX: usize = 1;
pub const CHARACTER_NAME_INDEX: usize = 0;
pub const FILE_NAME_INDEX: usize = 1;
pub const TOKEN_NUMBER: usize = 2;

/// Minimum number of lines in the config file
/// PS: Not on the instructions, Avoid using hardcoded numbers
pub const MIN_CONFIG_LINES: usize = 2;

pub(crate) struct Play {
    frags: Fragments,
}

impl Play {
    pub fn new() -> Self {
        Self {
            frags: Fragments::new(),
        }
    }

    /// Parse the `config_line` to type `ScriptConfig` and push it to `play_configs`
    pub fn add_config(config_line: &String, script_config: &mut ScriptConfig) {
        let tokens: Vec<&str> = config_line.split_whitespace().collect();
        if tokens.len() == 0 {
            // skip blank lines
            return;
        }

        match tokens[FIRST_TOKEN_INDEX] {
            "[scene]" => {
                match tokens.len() {
                    1 => {
                        if SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                            stderr_writeln!("ERROR: MISSING TITLE");
                        }
                    }
                    _ => {
                        let title = tokens[1..].join(" ");
                        script_config.push((true, title));
                    }
                }
            }
            _ => {
                script_config.push((false, tokens[FIRST_TOKEN_INDEX].to_string()));
                if tokens.len() > 1 && SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                    stderr_writeln!("WHINGE: EXTRA TOKEN DETECTED {}", config_line);
                }
            }
        }
    }

    /// Read the `config_file_name` file and parse it to `title` and `play_config`
    pub fn read_config(script_file: &String, script_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut script_lines: Vec<String> = Vec::new();

        // Read the config file to lines
        grab_trimmed_file_lines(script_file, &mut script_lines)?;
        // Will return Err(ERR_SCRIPT_GENERATION_FAIL) if the file can not be read

        for line in &script_lines {
            Play::add_config(line, script_config);
        }

        Ok(())
    }

    /// iterates through its vector of scene fragments and calls each one's enter then recite then exit methods, except that for the first
    /// fragment it should call enter_all instead of enter, and for the last fragment it should call exit_all instead of exit.
    pub fn recite(self: &mut Play) {
        if self.frags.len() == 0 {
            return;
        }

        for index in 0..self.frags.len() {
            match index {
                0 => {
                    mutex_lock_ref!(self.frags[index]).enter_all();
                    // recite the scenefrag
                    mutex_lock_mut!(self.frags[index]).recite();
                    match self.frags.len() {
                        1 => {
                            mutex_lock_ref!(self.frags[index]).exit_all();
                        }
                        _ => {
                            mutex_lock_ref!(self.frags[index]).exit(
                                mutex_lock_ref!(self.frags[index + 1])
                            ); // passing the next frag into exit}
                        }
                    }
                }
                _ => {
                    mutex_lock_ref!(self.frags[index]).enter(
                        mutex_lock_ref!(&self.frags[index - 1])
                    );
                    mutex_lock_mut!(self.frags[index]).recite();
                    // doing exit all for last one
                    if index == self.frags.len() - 1 {
                        mutex_lock_ref!(self.frags[index]).exit_all();
                    } else {
                        mutex_lock_ref!(self.frags[index]).exit(
                            mutex_lock_ref!(&self.frags[index + 1])
                        );
                    }
                }
            }
        }
    }

    /// Get the `title` and `play` from the `config_file_name` file
    pub fn prepare(self: &mut Self, script_file_name: &String) -> Result<(), u8> {
        let mut script_config: ScriptConfig = ScriptConfig::new();
        // Pass error code to the caller, unwrapping causig panic
        Self::read_config(script_file_name, &mut script_config)?;
        // process the config
        Self::process_config(self, &script_config)?;
        // i don't know if I need to do MIN_FRAG_NUM
        if self.frags.len() <= 0 || !mutex_lock_ref!(self.frags[FIRST_TOKEN_INDEX]).has_title() {
            stderr_writeln!(
                "ERROR: FIRST SCENE FRAGMENT DOES NOT HAVE A TITLE OR NO FRAGMENTS PARSED"
            );
            return Err(ERR_SCRIPT_GENERATION_FAIL);
        }
        Ok(())
    }

    /// Parse all lines in `ScriptConfig` and push it to `self.frags`
    pub fn process_config(self: &mut Play, config: &ScriptConfig) -> Result<(), u8> {
        // ScriptConfig: Vec[(name of character, file name), ...]
        // ScriptConfig: Vec<(bool, String)>
        let mut current_title = "".to_string();
        // all thread handles
        let mut handles: Vec<JoinHandle<SceneFragment>> = Vec::<JoinHandle<SceneFragment>>::new();
        for (flag, text) in config.into_iter() {
            match flag {
                true => {
                    // true means it's a new scene
                    current_title = text.clone(); // Move `text` directly into `current_title`
                }
                _ => {
                    let mut scene_frag = SceneFragment::new(&current_title);
                    current_title = "".to_string();
                    let config_name = text.clone();
                    let handle = thread::spawn(move || -> SceneFragment {
                        scene_frag.prepare(&config_name);
                        return scene_frag;
                    });
                    handles.push(handle);
                }
            }
        }

        // handles is Vec<JoinHandle<SceneFragment>>
        let mut idx = 0;
        for handle in handles {
            match handle.join() {
                Ok(frag) => {
                    self.frags.push(Arc::new(Mutex::new(frag)));
                    idx += 1;
                }
                Err(_) => {
                    eprintln!("error in thread {}", idx);
                    return Err(ERR_SCRIPT_GENERATION_FAIL);
                }
            }
        }
        Ok(())
    }
}
