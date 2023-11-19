pub trait Create {
    fn create(&self, title: &str) {
        println!("{} is bring created", title);
    }
}
