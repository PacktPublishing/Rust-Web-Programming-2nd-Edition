use std::{thread, time};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTask {
    pub one: i32,
    pub two: i32
}

impl AddTask {

    pub fn run(self) -> i32 {
        let duration = time::Duration::from_secs(20);
        thread::sleep(duration);
        return self.one + self.two
    }
}
