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
    pub fn new(input_title: &str) -> Pending {
        let base = Base::new(input_title, 
            TaskStatus::PENDING);
        return Pending{super_struct: base}
    }
}


#[cfg(test)]
mod pending_tests {

    use super::Pending;
    use super::TaskStatus;

    #[test]
    fn new() {
        let new_base_struct = Pending::new("test title");
        assert_eq!(String::from("test title"), new_base_struct.super_struct.title);
        assert_eq!(TaskStatus::PENDING, new_base_struct.super_struct.status);
    }
}
