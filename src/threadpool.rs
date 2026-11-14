use anyhow::Context;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // workers.push(Worker::new(id));

            match Worker::new(id) {
                Ok(worker) => workers.push(worker),
                Err(e) => eprintln!("Could not create worker. Error: {:?}", e),
            }
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f()
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread_handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> anyhow::Result<Worker> {
        let thread_handle = thread::Builder::new()
            .spawn(|| {})
            .context("Could not create thread")?;

        // let thread_handle = thread::spawn(|| {});

        Ok(Worker { id, thread_handle })
    }
}
