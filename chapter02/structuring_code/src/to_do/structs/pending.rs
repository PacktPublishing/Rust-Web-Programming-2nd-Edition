use super::base::Base;
use super::super::enums::TaskStatus;


/// This struct is responsible for defing a task that is yet to be completed. 
/// 
/// # Attributes 
/// * super_struct (Base): the super struct that hosts the core functionality and fields
pub struct Pending {
    pub super_struct: Base
}

impl Pending {

    /// The constructor for the Done struct. 
    /// 
    /// # Arguements
    /// * input_title (&str): the title of the pending task being created
    /// 
    /// # Returns 
    /// (Pending): the constructed struct
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct: base}
    }
}