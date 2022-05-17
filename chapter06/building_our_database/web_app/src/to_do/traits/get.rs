use serde_json::Map;
use serde_json::value::Value;


/// This trait enables a to-do item to get do-do tasks.
pub trait Get {

    /// Gets the item from our JSON map just prints it out for now. 
    /// 
    /// # Arguments 
    /// * title (&str): the title to be printed out
    /// * state (*Map<String, Value>): the current state of the to-do items
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            },
            None => println!("item: {} was not found",
            title)
        }
    }
}