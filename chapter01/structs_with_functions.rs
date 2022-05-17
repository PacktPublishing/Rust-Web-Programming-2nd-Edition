
#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friend: Friend
}
impl Human {
    
    fn new(name: &str, age: i8) -> Human {
        return Human{
            name: name.to_string(),
            age: age,
            current_thought: None,
            friend: Friend::NIL
        }
    }
    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = Some(thought.to_string());
        return self
    }
    fn with_friend(mut self, friend: Box<Human>) -> Human {
        self.friend = Friend::HUMAN(friend);
        return self
    }
    
}


fn main() {

    let developer_friend = Human::new("Caroline Morton", 30);
    
    let developer = Human::new("Maxwell Flitton", 32)
                           .with_thought("I love Rust!")
                           .with_friend(Box::new(developer_friend));
    println!("Name: {:?}", developer.name);
    println!("Age: {:?}", developer.age);
    println!("Current Thought: {:?}", developer.current_thought);
    println!("Friend: {:?}", developer.friend);
}

