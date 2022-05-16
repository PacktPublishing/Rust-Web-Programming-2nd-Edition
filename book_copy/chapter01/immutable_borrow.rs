

fn print(value: &String) {
    println!("{}", value);
}

fn print_two(value: &String, value_two: &String) {
    println!("{}", value);
    println!("{}", value_two);
}


fn main() {
    let one = "one".to_string();
    print(&one);
    println!("{}", one);

    let one = "one".to_string();
    print_two(&one, &one);
    println!("{}", one);

}
