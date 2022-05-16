

fn error_check(check: bool) -> Result<i8, &'static str> {
    if check == true {
        Err("this is an error")
        } 
    else {
        Ok(1)
    }
}


fn main() {
    println!("{:?}", error_check(false));
    println!("{:?}", error_check(false).is_err());
    println!("{:?}", error_check(true));
    println!("{:?}", error_check(true).is_err());
}
