pub mod structs;
pub mod enums;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;


pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str,
                     status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        }
    }
}
