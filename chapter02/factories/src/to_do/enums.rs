

/// This Enum is responsible for defining the current status of a task. 
/// 
/// # Attributes 
/// * DONE: when the task is finished 
/// * PENDING: when the task is waiting to be finished
pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {

    /// turns the TaskStatus enum into a string. 
    /// 
    /// # Returns 
    /// * (String): the status of the task
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {return "DONE".to_string()},
            &Self::PENDING => {return "PENDING".to_string()}
        }
    }
}
