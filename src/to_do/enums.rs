use std::fmt;
use std::fmt::{Formatter};
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Eq, Debug)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn new(status: &str) -> TaskStatus {
        match status {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", status),
        }
    }
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()},
        }
    }
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::DONE => { write!(f, "DONE") },
            &Self::PENDING => { write!(f, "PENDING") }
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TaskStatus::DONE => {
                match other {
                    &TaskStatus::DONE => true,
                    &TaskStatus::PENDING => false,
                }
            },
            TaskStatus::PENDING => {
                match other {
                    &TaskStatus::DONE => false,
                    &TaskStatus::PENDING => true,
                }
            }
        }
    }
}