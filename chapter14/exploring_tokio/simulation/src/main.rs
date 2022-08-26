use std::time::Instant;
use std::{thread, time};


async fn hello(input_int: i32) -> i32 {
    let five_seconds = time::Duration::from_secs(5);
    thread::sleep(five_seconds);
    println!("Hello, world! {}", input_int);
    return input_int
}


#[tokio::main]
async fn main() {
    let now = Instant::now();

    let one = tokio::spawn(async move {
        hello(1).await
    });
    let two = tokio::spawn(async move {
        hello(2).await
    });
    let three = tokio::spawn(async move {
        hello(3).await
    });
    one.await;
    two.await;
    three.await;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}