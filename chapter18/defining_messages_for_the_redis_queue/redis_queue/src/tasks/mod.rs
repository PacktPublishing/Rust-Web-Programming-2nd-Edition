pub mod add;
pub mod multiply;
pub mod subtract;

use serde::{Serialize, Deserialize};
use add::AddTask;
use multiply::MultiplyTask;
use subtract::SubtractTask;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    ADD(AddTask),
    MULTIPLY(MultiplyTask),
    SUBTRACT(SubtractTask)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMessage {
    pub task: TaskType
}
