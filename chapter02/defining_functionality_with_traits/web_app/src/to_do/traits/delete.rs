

pub trait Delete {
    fn delete(&self, title: &str) {
        println!("{} is being deleted", title);
    }
}
