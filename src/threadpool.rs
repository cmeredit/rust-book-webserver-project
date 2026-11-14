pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(_thread_count: usize) -> ThreadPool {
        ThreadPool {}
    }

    pub fn execute(&self, closure: Box<dyn FnOnce()>) {
        closure()
    }
}
