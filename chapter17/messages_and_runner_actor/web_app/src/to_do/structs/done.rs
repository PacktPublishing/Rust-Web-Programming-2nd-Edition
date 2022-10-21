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
        let base = Base::new(input_title, 
            TaskStatus::DONE);
        return Done{super_struct: base}
    }
}


#[cfg(test)]
mod done_tests {

    use super::Done;
    use super::TaskStatus;

    #[test]
    fn new() {
        let new_base_struct = Done::new("test title");
        assert_eq!(String::from("test title"), new_base_struct.super_struct.title);
        assert_eq!(TaskStatus::DONE, new_base_struct.super_struct.status);
    }
}
