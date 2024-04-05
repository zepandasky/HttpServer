use crate::threading::{worker::Worker, Job};
use std::sync::{mpsc, Arc, Mutex};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();

        //Multiple Sender single consumer so a RC is needed but for multithreading its ARC with Mutex
        let rx = Arc::new(Mutex::new(rx));

        // Create the workers and give every worker a receiving end of the channel
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // Join the thread if the ThreadPool is  being shutdown
            if let Some(worker_thread) = worker.thread.take() {
                worker_thread.join().unwrap();
            }
        }
    }
}
