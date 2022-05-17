pub mod structs;
pub mod enums;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;


/// This struct is responsible for packaging to-do structs to be returned. 
/// 
/// # Attributes 
/// * Pending (Pending): a pending item struct 
/// * Done (Done): a done item struct
pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}


/// This function builds to-do structs based on the status passed in. 
/// 
/// # Arguments 
/// * title (&str): the title for the to do item to be made 
/// * status (TaskStatus): the type of status for the to-do item being created
pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            return ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            return ItemTypes::Pending(Pending::new(title))
        }
    }
}