use serde_json::{json, Map, Value};
use crate::state::write_to_file;
use crate::to_do::enums::TaskStatus;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>, file_path: &str) {
        state.insert(title.to_string(), json!(TaskStatus::DONE.stringify()));
        write_to_file(file_path, state);
        println!("\n\n{} is being set to done\n\n", title);
    }
    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>, file_path: &str) {
        state.insert(title.to_string(), json!(TaskStatus::PENDING.stringify()));
        write_to_file(file_path, state);
        println!("{} is being set to pending", title);
    }
}