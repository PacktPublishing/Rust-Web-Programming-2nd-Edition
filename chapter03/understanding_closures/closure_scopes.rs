

fn main() {
    {
        let test_closure = |string_input| {
            println!("{}", string_input);
        };
    }
    test_closure("test");
}
