
use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
  

    pub fn execute<F>(&self, f: F)
    where
        // FnOnce() is the trait we want to use because the thread for running a request will only execute that request’s closure one time
        // We need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute. 
        F: FnOnce() + Send + 'static,
    {
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }
}