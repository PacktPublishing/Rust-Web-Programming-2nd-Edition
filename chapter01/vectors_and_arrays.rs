

enum SomeValue {
    StringValue(String),
    IntValue(i32)
}


fn main() {
    let int_array: [i32; 3] = [1, 2, 3];
    for i in int_array.iter() {
        println!("{}", i);
    }

    let mut mutable_array: [i32; 3] = [1, 2, 0];
    mutable_array[2] = 3;
    println!("{:?}", mutable_array);
    println!("{}", mutable_array.len());

    let slice_array: [i32; 100] = [0; 100];
    println!("length: {}", slice_array.len());
    println!("slice: {:?}", &slice_array[5 .. 8]);

    for i in multi_array.iter() {
        match i {
            SomeValue::StringValue(data) => {
                println!("The string is: {}", data);
            },
            SomeValue::IntValue(data) => {
                println!("The int is: {}", data);
            }
        }
    }
    
    let mut string_vector: Vec<&str> = vec!("one", "two", "three");
    println!("{:?}", string_vector);
    string_vector.push("four");
    println!("{:?}", string_vector);
    let _empty_vector: Vec<&str> = Vec::new();
}