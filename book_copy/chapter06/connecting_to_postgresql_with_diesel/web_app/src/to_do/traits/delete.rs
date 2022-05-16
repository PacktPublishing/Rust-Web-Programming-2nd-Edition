use serde_json::Map;
use serde_json::value::Value;

use crate::state::write_to_file;


/// This trait is responsible for enabling an item trait to delete a task. 
pub trait Delete {

    /// This function deletes a task from our to-do items and then writing it to a JSON file.
    /// 
    /// # Arguments 
    /// * title (&str): title of the task to be deleted from the state
    /// * state (*Map<String, Value>): the current state of the to-do items
    fn delete(&self, title: &String,
        state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
