

fn print(value: &mut i8, value_two: &mut i8) {
    *value += 1;
    println!("In function the value is: {}", value);
    *value_two += 1;
}

fn main() {
    let mut one: i8 = 5;
    // should break below
    print(&mut one, &mut one);
    println!("In main the value is: {}", one);
}
