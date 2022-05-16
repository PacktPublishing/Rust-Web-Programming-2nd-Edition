use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;
use super::super::enums::TaskStatus;


/// This trait enables a to-do item struct to edit the status. 
pub trait Edit {

    /// This function is for setting a task to done writing it to JSON file. 
    /// 
    /// # Arguments 
    /// * title (&str): the title of the task being printed out 
    /// * state (*Map<String, Value>): the current state of the to-do items
    fn set_to_done(&self, title: &String,
        state: &mut Map<String, Value>) {
        state.insert(title.to_string(),
        json!(TaskStatus::DONE.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    /// This function is for setting a task to pending writing it to JSON file. 
    /// 
    /// # Arguments 
    /// * title (&str): the title of the task being printed out
    /// * state (*Map<String, Value>): the current state of the to-do items
    fn set_to_pending(&self, title: &String,
        state: &mut Map<String, Value>) {
        state.insert(title.to_string(),
        json!(TaskStatus::PENDING.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to pending\n\n", title);
    }

}
