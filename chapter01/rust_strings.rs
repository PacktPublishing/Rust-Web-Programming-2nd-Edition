fn print(message: String) {
    println!("{}", message);
}

fn main() {
    let string_one = "hello world".to_owned();
    let string_two = "hello world".to_string();
    let string_three = string_two.clone();
    let message = String::from("hello world");
    print(message);
    print(&"hello world");
}