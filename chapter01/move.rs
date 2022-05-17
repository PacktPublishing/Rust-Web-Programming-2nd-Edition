

fn main() {
    let one: String = String::from("one");
    let two: String = one.to_owned() + " two";
    println!("{}", two);
    println!("{}", one);
} 