

pub trait Get {
    fn get(&self, title: &str) {
        println!("{} is being fetched", title);
    }
}
