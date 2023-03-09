pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::pending::Pending;
use structs::done::Done;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        },
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        },
    }
}