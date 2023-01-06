use futures::executor::block_on;
use std::{thread, time};
use futures::join;


async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}


fn main() {
    let now = time::Instant::now();
    let future_two = async {
        return do_something(2).await
    };
    let future_two = block_on(future_two);
    println!("Here is the outcome: {:?}", future_two);
    let future_three = async {
        let outcome_one = do_something(2).await;
        let outcome_two = do_something(3).await;
        return outcome_one + outcome_two
    };
    let future_four = async {
        let outcome_one = do_something(2);
        let outcome_two = do_something(3);
        let results = join!(outcome_one, outcome_two);
        return results.0 + results.1
    };

    let now = time::Instant::now();
    let result = block_on(future_four);
    println!("time elapsed {:?}", now.elapsed());
    println!("here is the result: {:?}", result);
}
