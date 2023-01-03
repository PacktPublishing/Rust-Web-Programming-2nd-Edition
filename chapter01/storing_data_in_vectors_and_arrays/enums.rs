

enum SomeValue {
    StringValue(String),
    IntValue(i32)
}


fn main() {
    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(2),
        SomeValue::StringValue(String::from("three")),
        SomeValue::IntValue(4)
    ];
    for i in multi_array {
        match i {
            SomeValue::StringValue(data) => {
                println!("The string is: {}", data);
            },
            SomeValue::IntValue(data) => {
                println!("The int is: {}", data);
            }
        }
    }
}
