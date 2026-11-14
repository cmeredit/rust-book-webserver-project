pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(_thread_count: usize) -> ThreadPool {
        ThreadPool {}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f()
    }
}
