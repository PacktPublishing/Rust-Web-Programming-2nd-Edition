

/// This trait is responsible for enabling an item trait to delete a task. 
pub trait Delete {

    /// This function deletes a task however, for now we are just printing it out.  
    /// 
    /// # Arguments 
    /// * title (&str): title of the task to be printed out
    fn delete(&self, title: &str) {
        println!("{} is being deleted", title);
    }

}
