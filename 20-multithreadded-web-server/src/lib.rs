use std::thread;
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl Worker {
    /// Spawn a new thread for a Worker
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
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
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // NOTE: CHALLENGE to create a build() function that can have error handling
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
