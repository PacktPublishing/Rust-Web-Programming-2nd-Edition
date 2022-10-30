use serde::{Serialize, Deserialize};

pub mod add;
pub mod multiply;
pub mod subtract;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    ADD,
    MULTIPLY,
    SUBTRACT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMessage {
    pub task_type: TaskType,
    pub task: Vec<u8>
}

