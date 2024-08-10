pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_num_threads: i32) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, _func: F)
    where
        F: FnOnce() + Send + 'static
    {}
}
