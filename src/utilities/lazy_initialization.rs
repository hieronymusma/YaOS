pub struct LazyInitializer<T, F: FnOnce() -> T> {
    value: Option<T>,
    init_function: F
}

impl<T, F: FnOnce() -> T> LazyInitializer<T, F> {
    
}