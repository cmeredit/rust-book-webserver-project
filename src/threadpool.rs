pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(_thread_count: u8) -> ThreadPool {
        ThreadPool {}
    }

    pub fn execute(&self, closure: Box<dyn FnOnce()>) {
        closure()
    }
}
