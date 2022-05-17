use super::super::enums::TaskStatus;


/// This is the base struct to act as inheritance by composition for other structs. 
/// 
/// # Attributes 
/// * title (String): the title of the task 
/// * status (TaskStatus): the status of the task
pub struct Base {
    pub title: String,
    pub status: TaskStatus
}

impl Base {

    /// The constructor for the Base struct. 
    /// 
    /// # Arguments 
    /// * input_title (&str): the title of the task being created
    /// * input_status (TaskStatus): the status of the task being created 
    /// 
    /// # Returns 
    /// * (Base): the constructed base
    pub fn new(input_title: &str, input_status: TaskStatus) -> Base {
        return Base { 
            title: input_title.to_string(), 
            status: input_status 
        }
    }
}
