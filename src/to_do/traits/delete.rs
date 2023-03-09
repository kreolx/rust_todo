use serde_json::{Map, Value};
use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>, file_path: &str) {
        state.remove(title);
        write_to_file(file_path, state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}