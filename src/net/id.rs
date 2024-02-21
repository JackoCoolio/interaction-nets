use std::sync::atomic::AtomicUsize;

pub struct IdAllocator {
    next_id: AtomicUsize,
}

impl IdAllocator {
    pub fn new() -> Self {
        Self::new_at(0)
    }

    pub fn new_at(next_id: usize) -> Self {
        Self {
            next_id: AtomicUsize::new(next_id),
        }
    }

    pub unsafe fn set_next_id(&self, value: usize) {
        self.next_id
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    /// Gets the next available ID.
    pub fn create_id(&self) -> usize {
        self.next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    /// Retires an ID. Calling this is technically necessary.
    pub fn retire_id(&self, id: usize) {
        // will need a lock while this function is executing
        _ = id;
    }
}
