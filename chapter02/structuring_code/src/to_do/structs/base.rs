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

