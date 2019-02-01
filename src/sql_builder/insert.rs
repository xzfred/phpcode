// use std::option::Option;
// use std::string::ToString;

// #[macro_use]
use json;

#[derive(Clone, Debug)]
pub struct Insert {
    table: Option<String>,
    columns: Option<json::JsonValue>,
}

impl Insert {
    pub fn new() -> Self {
        Insert {
            table: None,
            columns: None,
        }
    }
}

impl ToString for Insert {
    fn to_string(&self) -> String {
        String::from("test")
    }
}
