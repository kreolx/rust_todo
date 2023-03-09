use serde_json::Map;
use serde_json::value::Value;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use super::to_do::ItemTypes;
use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::traits::get::Get;
use super::to_do::traits::create::Create;
use super::to_do::traits::delete::Delete;
use super::to_do::traits::edit::Edit;

fn process_pending(item: Pending, command: String, state: &Map<String, Value>, file_path: &str) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status.stringify(), &mut state, file_path),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state, file_path),
        _ => println!("command: {} not supported", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>, file_path: &str) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state, file_path),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state, file_path),
        _ => println!("command: {} not supported", command),
    }
}

pub fn process_input(title: String, command: String, state: &Map<String, Value>, file_path: &str) {
    match command.as_str() {
        "all" => {
            for (key, value) in *&state {
                println!("{} {}", key, value);
            }
        },
        _ => {
            let status: String;
            match &state.get(&title) {
                Some(result) => {
                    status = result.to_string().replace('\"', "");
                },
                None => {
                    status = "pending".to_owned();
                }
            }
            let item = to_do_factory(title.as_str(), TaskStatus::from_string(status.to_uppercase()));
            match item {
                ItemTypes::Pending(item) => process_pending(item, command, state, file_path),
                ItemTypes::Done(item) => process_done(item, command, state, file_path),
            }
        }
    }

}