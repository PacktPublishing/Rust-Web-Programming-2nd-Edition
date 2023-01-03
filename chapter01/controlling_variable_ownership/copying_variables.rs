

fn main() {
    let one: i8 = 10;
    let two: i8 = one + 5;
    println!("{}", one);
    println!("{}", two);

    let one = "one".to_string();
    let two = one;
    // should break below
    println!("{}", one);
    println!("{}", two);
}