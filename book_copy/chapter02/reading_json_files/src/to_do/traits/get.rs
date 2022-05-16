


/// This trait enables a to-do item to get do-do tasks.
pub trait Get {

    /// Gets the item and merely just prints it out for now. 
    /// 
    /// # Arguments 
    /// * title (&str): the title to be printed out
    fn get(&self, title: &str) {
        println!("{} is being fetched", title);
    }
}