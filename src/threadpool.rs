use anyhow::Context;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // workers.push(Worker::new(id));

            match Worker::new(id, Arc::clone(&receiver)) {
                Ok(worker) => workers.push(worker),
                Err(e) => eprintln!("Could not create worker. Error: {:?}", e),
            }
        }

        ThreadPool {
            _workers: workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(job)
            .expect("Threadpool should be able to send job");
    }
}

#[derive(Debug)]
struct Worker {
    _id: usize,
    _thread: JoinHandle<()>,
    // receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> anyhow::Result<Worker> {
        let thread = thread::Builder::new()
            .spawn(move || {
                loop {
                    // Get that dang ol job
                    let job = receiver
                        .lock()
                        .expect("Mutex should not be in poisoned state")
                        .recv()
                        .expect("Channel should be open");
                    println!("Worker {id} got a job; executing.");
                    job();
                }
            })
            .context("Could not create thread")?;

        // let thread_handle = thread::spawn(|| {});

        Ok(Worker {
            _id: id,
            _thread: thread,
        })
    }
}
