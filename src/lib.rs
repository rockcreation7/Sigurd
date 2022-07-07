pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        // FnOnce() is the trait we want to use because the thread for running a request will only execute that request’s closure one time
        // We need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute. 
        F: FnOnce() + Send + 'static,
    {
    }
}