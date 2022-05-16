

/// This trait is responsible for enabling a struct to be created.  
pub trait Create {

    /// This function is responsible for creating a to-do item. However, for now we are just printing it 
    /// out
    /// 
    /// # Arguments 
    /// * title (&str): the title of the task to be printed out
    fn create(&self, title: &str) {
        println!("{} is being created", title);
    }

}
