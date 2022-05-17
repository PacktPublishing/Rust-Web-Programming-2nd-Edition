use super::base::Base;
use super::super::enums::TaskStatus;

use super::super::traits::get::Get;
use super::super::traits::edit::Edit;
use super::super::traits::create::Create;


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

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}