

fn print(value: &String) {
    println!("{}", value);
}

fn main() {
    let one = "one".to_string();
    print(&one);
    println!("{}", one);
}
