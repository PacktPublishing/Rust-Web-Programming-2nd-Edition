use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path: &str = &args[0];

    if path.contains("/debug/") {
        println!("Debug is running");
    }
    else if path.contains("/release/") {
        println!("release is running");
    }
    else {
        panic!("The setting is neither debug or release");
    }
}
