pub struct Address<T> {
    ptr: Unique<T>,
    cap: usize,
    len: usize,
}

impl<T> Address<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec { ptr: Unique::dangling(), len: 0, cap: 0 }
    }
}