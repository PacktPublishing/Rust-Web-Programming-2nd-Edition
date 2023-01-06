use std::vec::Vec;
use async_std;
use futures::future::join_all;
use futures::executor::block_on;

use std::{thread, time};
use std::sync::Arc;
use std::sync::Mutex;


async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}


fn main() {
    let async_outcome = async {
        // 1.
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);

        // 2.
        futures_vec.push(future_four);
        futures_vec.push(future_five);

        // 3.
        let handles = futures_vec.into_iter().map(
            async_std::task::spawn).collect::<Vec<_>>();

        // 4.
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let now = time::Instant::now();
    let result = block_on(async_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);

    let names = Arc::new(vec!["dave", "chloe", "simon"]);
    let reference_data = Arc::clone(&names);

    let new_thread = thread::spawn(move || {
        println!("{}", reference_data[1]);
    });

    let count = Mutex::new(0);

    let new_thread = thread::spawn(move || {
        *count.lock().unwrap() += 1;
    });

}