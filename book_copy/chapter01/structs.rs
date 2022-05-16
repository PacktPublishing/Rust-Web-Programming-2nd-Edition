
#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
    current_thought: String,
    friend: Friend
}

fn main() {

    let another_developer = Human{
        name: "Caroline Morton".to_string(),
        age:30,
        current_thought: "I need to code!!".to_string(),
        friend: Friend::NIL
    };

    let developer = Human{
        name: "Maxwell Flitton".to_string(),
        age: 32,
        current_thought: "nothing".to_string(),
        friend: Friend::HUMAN(Box::new(another_developer))
    };
    match &developer.friend {
        Friend::HUMAN(data) => {
            println!("{}", data.name);
        },
        Friend::NIL => {}
    }
}

