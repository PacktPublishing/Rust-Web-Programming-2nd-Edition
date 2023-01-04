

fn main() {
    let another_str = "case";
    let test_closure = |string_input| {
        println!("{} {}", string_input, another_str);
    };
    test_closure("test");
}
