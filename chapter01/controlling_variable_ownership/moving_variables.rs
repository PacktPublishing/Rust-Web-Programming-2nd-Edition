

fn main() {
    let one: String = String::from("one");
    let two: String = one + " two";
    println!("{}", two);
    // should break below
    println!("{}", one);

}
