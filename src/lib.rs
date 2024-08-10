#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSizeError,
}

pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, _func: F)
    where
        F: FnOnce() + Send + 'static
    {}

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroSizeError)
        }

        Ok(ThreadPool::new(size))
    }
}
