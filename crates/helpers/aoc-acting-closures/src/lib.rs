pub trait ActingClosures<T, U> {
    fn apply<F>(self, closure: F) -> U where F: Fn(T) -> U;
    fn also<F>(self, closure: F) -> T where F: Fn(&T);
}

impl <T, U> ActingClosures<T, U> for T {
    fn apply<F>(self, closure: F) -> U where F: Fn(T) -> U {
        closure(self)
    }

    fn also<F>(self, closure: F) -> T where F: Fn(&T) {
        closure(&self);
        self
    }
}