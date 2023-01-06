mod to_do;
mod state;

use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::ItemTypes;

use crate::to_do::traits::get::Get;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;

use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}

