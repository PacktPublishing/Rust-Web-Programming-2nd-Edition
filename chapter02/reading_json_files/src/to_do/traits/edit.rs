

/// This trait enables a to-do item struct to edit the status. 
pub trait Edit {

    /// This function is for setting a task to done but for now we are just printing it out. 
    /// 
    /// # Arguments 
    /// * title (&str): the title of the task being printed out 
    fn set_to_done(&self, title: &str) {
        println!("{} is being set to done", title);
    }

    /// This function is for setting a task to pending but for now we are just printing it out. 
    /// 
    /// # Arguments 
    /// * title (&str): the title of the task being printed out
    fn set_to_pending(&self, title: &str) {
        println!("{} is being set to pending", title);
    }

}
