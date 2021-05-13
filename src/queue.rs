use std::alloc::{self, Layout};

pub struct Queue<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
    front: usize,
    back: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let ptr = unsafe {
            alloc::alloc(Self::layout(4)) as *mut T
        };

        Queue { ptr, cap: 4, len: 0, front: 0, back: 0 }
    }

    pub fn enqueue(&mut self, elem: T) {
        if self.cap == self.len {
            self.resize(self.cap * 2);
        }

        unsafe {
            self.ptr.add(self.back).write(elem);
        }

        self.back = (self.back + 1) % self.cap;
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let elem = unsafe {
            self.ptr.add(self.front).read()
        };

        self.front = (self.front + 1) % self.cap;
        self.len -= 1;

        if self.cap > self.len * 2 && self.cap > 4 {
            self.resize(self.cap / 2);
        }

        Some(elem)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn resize(&mut self, new_cap: usize) {
        let ptr = unsafe {
            alloc::alloc(Self::layout(new_cap)) as *mut T
        };

        for write_at in 0..self.cap {
            let read_at = (self.front + write_at) % self.cap;
            unsafe {
                ptr.add(write_at).write(self.ptr.add(read_at).read());
            }
        }

        unsafe {
            alloc::dealloc(self.ptr as *mut u8, Self::layout(self.cap));
        }

        self.ptr = ptr;
        self.cap = new_cap;
        self.front = 0;
        self.back = self.len;
    }

    fn layout(cap: usize) -> Layout {
        Layout::array::<T>(cap).unwrap()
    }
}


impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, Self::layout(self.cap));
        }
    }
}


#[cfg(test)]
mod test {
    use crate::queue::Queue;

    #[test]
    fn basics() {
        let mut queue = Queue::new();

        // Check empty queue behaves right
        assert_eq!(queue.dequeue(), None);

        // Populate queue
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        // Check normal removal
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.enqueue(4);
        queue.enqueue(5);

        // Check normal removal
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));

        // Check exhaustion
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), None);

        // Check the exhaustion case fixed the pointer right
        queue.enqueue(6);
        queue.enqueue(7);

        // Check normal removal
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        queue.enqueue(6);
        queue.enqueue(7);
        queue.enqueue(8);

        assert_eq!(queue.len(), 8);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.dequeue(), None);
    }
}