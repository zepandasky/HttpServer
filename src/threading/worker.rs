use crate::threading::Job;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct Worker {
    pub thread: Option<thread::JoinHandle<()>>,
    pub id: usize,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        Worker {
            thread: Some(thread),
            id: id,
        }
    }
}
