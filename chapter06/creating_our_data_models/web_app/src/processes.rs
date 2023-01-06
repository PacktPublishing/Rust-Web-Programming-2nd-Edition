use serde_json::Map;
use serde_json::value::Value;
use super::to_do::ItemTypes;
use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::traits::get::Get;
use super::to_do::traits::create::Create;
use super::to_do::traits::delete::Delete;
use super::to_do::traits::edit::Edit;


fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title,
                                &item.super_struct.status.stringify(), &mut state),
        "edit" => item.set_to_done(&item.super_struct.title,
                                   &mut state),
        _ => println!("command: {} not supported", command)
    }
}


fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command)
    }
}


pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state)
    }
}
