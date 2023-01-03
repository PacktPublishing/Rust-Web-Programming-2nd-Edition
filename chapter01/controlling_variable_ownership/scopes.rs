

fn main() {
    let one = &"one";
    let two: &str;
    {
        println!("{}", one);
        two = &"two";
    }
    println!("{}", one);
    println!("{}", two);
}
