use std::{collections::HashMap, time::{Duration, Instant}};

//Three screens
pub enum CurrentScreen{
    Main,
    Editing,
    Exiting,
    Warning,
}

pub enum CurrentlyEditing{
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>, // key,value pair with serde support
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub warning_time: Option<Instant>,
}
impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            warning_time: None,
        }
    }

    pub fn save_key_value(&mut self) -> bool {
        if self.key_input.trim().is_empty() {
            false;
        }
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        
        //Reset editing variables after appending
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
        self.warning_time = None;
        true
    }

    pub fn check_warning_timeout(&mut self) {
        if let Some(warning_start) = self.warning_time {
            if warning_start.elapsed() >= Duration::from_secs(2) {
                self.current_screen = CurrentScreen::Editing;
                self.warning_time = None;
            }
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value), // Proceed to value
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key), // Proceed to key
            }
        }
        else {
            self.currently_editing = Some(CurrentlyEditing::Key); //Start with editing key
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()>{
        let output = serde_json::to_string(&self.pairs)?;
        println!("{:?}", output);
        Ok(())
    }
}

