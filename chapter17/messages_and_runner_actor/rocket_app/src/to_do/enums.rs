use serde::ser::{Serialize, Serializer};


/// This Enum is responsible for defining the current status of a task. 
/// 
/// # Attributes 
/// * DONE: when the task is finished 
/// * PENDING: when the task is waiting to be finished
#[derive(Clone, Eq, Debug)]
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

    pub fn new(status: &str) -> TaskStatus {
        match status {
            "DONE" => {return TaskStatus::DONE},
            "PENDING" => {return TaskStatus::PENDING},
            _ => {panic!("{} not supported", status)}
        }
    }
}

impl Serialize for TaskStatus {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}

impl PartialEq for TaskStatus {

    fn eq(&self, other: &Self) -> bool {
        match self {
            TaskStatus::DONE => {
                match other {
                    &TaskStatus::DONE => return true,
                    &TaskStatus::PENDING => false
                }
            },
            TaskStatus::PENDING => {
                match other {
                    &TaskStatus::DONE => return false,
                    &TaskStatus::PENDING => true
                }
            }
        }    
    }
}
