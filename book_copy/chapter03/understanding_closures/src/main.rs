use std::thread;
use std::time::Duration;


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


struct Cache<T> where T: Fn(u32) -> u32{
    calculation: T,
    value: Option<u32>
}

// look into generic arguments for implementation
impl<T> Cache<T> where T: Fn(u32) -> u32 {

    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v); // caching the return value 
                v
            }
        }
    }
}


fn add_doubles(closure: fn(i32) -> i32, 
               one: i32, two: i32) -> i32 {
    return closure(one) + closure(two)
}

fn main() {

    // defining a basic closure
    let test_closure = |string_input| {
        println!("{}", string_input);
        };
    test_closure("test");

    let test = String::from("test");
    
    let closure = |int_input| {
        return int_input * 2
    };
    let outcome = add_doubles(closure, 2, 3);
    println!("{}", outcome);
}
