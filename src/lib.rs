pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, _func: F)
    where
        F: FnOnce() + Send + 'static
    {}
}
