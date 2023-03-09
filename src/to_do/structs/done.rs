use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::get::Get;
use super::super::traits::edit::Edit;
use super::super::traits::delete::Delete;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        return Done { super_struct: base };
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}


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