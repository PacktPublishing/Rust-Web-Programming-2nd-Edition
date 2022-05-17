use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;


/// This trait is responsible for enabling a struct to be created.  
pub trait Create {

    /// This function is responsible for creating a to-do item and writes it to a JSON file.
    /// 
    /// # Arguments 
    /// * title (&str): the title of the task to be printed out
    /// * state (*Map<String, Value>): the current state of the to-do items
    fn create(&self, title: &String, status: &String,
        state: &mut Map<String, Value>) {
            state.insert(title.to_string(), json!(status));
            write_to_file("./state.json", state);
            println!("\n\n{} is being created\n\n", title);
    }
}
