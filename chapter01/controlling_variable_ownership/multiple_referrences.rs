

fn print(value: &String, value_two: &String) {
    println!("{}", value);
    println!("{}", value_two);
}

fn main() {
    let one = "one".to_string();
    print(&one, &one);
    println!("{}", one);
}
