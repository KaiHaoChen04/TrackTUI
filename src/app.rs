

//Three screens
pub enum CurrentScreen{
    Main,
    Editing,
    Exiting,
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
}