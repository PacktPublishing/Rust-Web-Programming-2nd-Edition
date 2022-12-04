use super::base::Base;
use super::super::enums::TaskStatus;


/// This struct is responsible for defing a completed task. 
/// 
/// # Attributes 
/// * super_struct (Base): the super struct that hosts the core functionality and fields
pub struct Done {
    pub super_struct: Base
}

impl Done {

    /// The constructor for the Done struct. 
    /// 
    /// # Arguements
    /// * input_title (&str): the title of the completed task being created
    /// 
    /// # Returns 
    /// (Done): the constructed struct
    pub fn new(input_title: &str) -> Done {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        return Done{super_struct: base}
    }
}
