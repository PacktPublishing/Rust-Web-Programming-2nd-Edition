use super::super::enums::TaskStatus;
use serde::Serialize;


/// This is the base struct to act as inheritance by composition for other structs. 
/// 
/// # Attributes 
/// * title (String): the title of the task 
/// * status (TaskStatus): the status of the task
#[derive(Serialize)]
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

#[cfg(test)]
mod base_tests {

    use super::Base;
    use super::TaskStatus;

    #[test]
    fn new() {
        let expected_title = String::from("test title");
        let expected_status = TaskStatus::DONE;
        let status = TaskStatus::DONE;
        let new_base_struct = Base::new("test title", status);
        assert_eq!(expected_title, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }
}