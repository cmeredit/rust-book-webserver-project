use anyhow::Context;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .expect("Sender should be available for entire life of owning ThreadPool")
            .send(job)
            .expect("Threadpool should be able to send job");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Take the sender (replacing contained value with None) and drop it
        // Workers will finish their current job and exit upon next call to recv
        // (they recognize that the channel has closed and exit)
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            if let Err(e) = worker.thread.join() {
                eprintln!(
                    "Failed to join worker with id {}. Associated thread panicked: {:?}",
                    worker.id, e
                );
            };
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
    // receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> anyhow::Result<Worker> {
        let thread = thread::Builder::new()
            .spawn(move || {
                loop {
                    // Wait for a message...
                    // If the channel is open, this blocks and waits for an Ok(message).
                    // If the channel is closed, we get an Err(_)
                    let message = receiver
                        .lock()
                        .expect("Mutex should not be in poisoned state")
                        .recv();

                    match message {
                        // There's always more work to do!
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
                            job();
                        }
                        // Channel has closed, so it's time to rest
                        Err(_) => {
                            println!("Worker {id} disconnected; shutting down.");
                            break;
                        }
                    }
                }
            })
            .context("Could not create thread")?;

        // let thread_handle = thread::spawn(|| {});

        Ok(Worker { id, thread })
    }
}
