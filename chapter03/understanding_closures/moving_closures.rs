

fn add_doubles(closure: fn(i32) -> i32,
               one: i32, two: i32) -> i32 {
    return closure(one) + closure(two)
}


fn main() {
    let closure = |int_input| {
        return int_input * 2
    };
    let outcome = add_doubles(closure, 2, 3);
    println!("{}", outcome);
}
