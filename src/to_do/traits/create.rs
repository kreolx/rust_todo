use serde_json::{json, Map, Value};
use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>, file_path: &str) {
        state.insert(title.to_string(), json!(status));
        write_to_file(file_path, state);
        println!("\n\n{} is being created\n\n", title);
    }
}