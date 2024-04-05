pub mod threadpool;
mod worker;


// Type definitionsq
pub type Job = Box<dyn FnOnce() + Send + 'static>;