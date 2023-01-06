

fn add_doubles(closure: Box<dyn Fn(i32) -> i32>,
               one: i32, two: i32) -> i32 {
    return closure(one) + closure(two)
}


fn main() {
    let one = 2;
    let closure = move |int_input| {
        return int_input * one
    };
    let outcome = add_doubles(Box::new(closure), 2, 3);
    println!("{}", outcome);
}
