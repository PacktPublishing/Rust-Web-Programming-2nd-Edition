fn print(message: &str) {
    println!("{}", message);
}

fn main() {
    print(&"hello world");
}
