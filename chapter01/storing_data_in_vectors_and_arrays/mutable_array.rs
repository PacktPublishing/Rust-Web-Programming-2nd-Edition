

fn main() {
    let mut mutable_array: [i32; 3] = [1, 2, 0];
    mutable_array[2] = 3;
    println!("{:?}", mutable_array);
    println!("{}", mutable_array.len());
}
