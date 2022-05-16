

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

    /// Creates an Enum from a String. 
    /// 
    /// # Arguments 
    /// * status (String): the string that the enum is going to be created from 
    /// 
    /// # Returns 
    /// * (TaskStatus): the constructed Enum
    pub fn from_string(status: String) -> TaskStatus {
        match status.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("task status {} is not supported", status)
        }
    }
}
