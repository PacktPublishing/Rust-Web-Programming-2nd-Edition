

fn print(value: &mut i8) {
    *value += 1;
    println!("In function the value is: {}", value);
}

fn main() {
    let mut one: i8 = 5;
    print(&mut one);
    println!("In main the value is: {}", one);
}
