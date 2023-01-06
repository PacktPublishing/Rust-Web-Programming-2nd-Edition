

#[derive(Debug)]
struct Human<'a> {
    name: &'a str,
    age: i8,
    current_thought: &'a str
}


fn main() {
    let developer = Human{
        name: "Maxwell Flitton",
        age: 32,
        current_thought: "nothing"
    };
    println!("{:?}", developer);
    println!("{}", developer.name);
}
