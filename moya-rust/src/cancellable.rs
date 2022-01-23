pub trait Cancellable {
    fn is_cancelled(&self) -> bool;
    fn cancel(&mut self);
}

pub struct CancellableWrapper {
    innerCancellable: SimpleCancellable
}
impl CancellableWrapper {
   pub fn new() -> Self {
        CancellableWrapper {
            innerCancellable: SimpleCancellable::new()
        }
    }
}
impl Cancellable for CancellableWrapper {
    fn is_cancelled(&self) -> bool {
        self.innerCancellable.is_cancelled
    }
    fn cancel(&mut self) {
        self.innerCancellable.cancel()
    }
}



pub struct SimpleCancellable {
    is_cancelled: bool
}

impl SimpleCancellable {
    fn new() -> Self {
        SimpleCancellable {
            is_cancelled: false
        }
    }
}
impl Cancellable for SimpleCancellable {
    fn is_cancelled(&self) -> bool {
        self.is_cancelled
    }
    fn cancel(&mut self) {
        self.is_cancelled = true
    }
}