use super::super::enums::TaskStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}


#[cfg(test)]
mod base_test {
    use super::Base;
    use super::TaskStatus;
    #[test]
    fn new() {
        let expected_tile = String::from("test title");
        let expected_status = TaskStatus::DONE;
        let new_base_struct = Base {
            title: expected_tile.clone(),
            status: TaskStatus::DONE,
        };
        assert_eq!(expected_tile, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }
}