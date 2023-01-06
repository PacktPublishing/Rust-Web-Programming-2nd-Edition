use std::collections::HashMap;


#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>)
}


fn main() {
    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();
    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert("items", CharacterValue::Items(vec![
        "laptop".to_string(),
        "book".to_string(),
        "coat".to_string()
    ]));
    println!("{:?}", profile);

    match profile.get("name") {
        Some(value_data) => {
            match value_data {
                CharacterValue::Name(name) => {
                    println!("the name is: {}", name);
                },
                _ => panic!("name should be a string")
            }
        },
        None => {
            println!("name is not present");
        }
    }

    match profile.get("name").unwrap() {
        CharacterValue::Name(name) => {
            println!("the name is: {}", name);
        },
        _ => panic!("name should be a string")
    }
}
