use std::alloc::{self, Layout};

pub struct Deque<T> {
    ptr: *mut T,
    cap: isize,
    len: isize,
    front: isize,
    back: isize,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        let ptr = unsafe {
            alloc::alloc(Self::layout(4)) as *mut T
        };

        Deque { ptr, cap: 4, len: 0, front: 0, back: 0 }
    }

    pub fn push_back(&mut self, elem: T) {
        if self.cap == self.len {
            self.resize(self.cap * 2);
        }

        unsafe {
            self.ptr.offset(self.back).write(elem);
        }

        self.back = (self.back + 1) % self.cap;
        self.len += 1;
    }

    pub fn push_front(&mut self, elem: T) {
        if self.cap == self.len {
            self.resize(self.cap * 2);
        }

        unsafe {
            self.ptr.offset((self.front - 1) % self.cap).write(elem);
        }

        self.front = (self.front - 1) % self.cap;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let elem = unsafe {
            self.ptr.offset(self.front).read()
        };

        self.front = (self.front + 1) % self.cap;
        self.len -= 1;

        if self.cap > self.len * 2 && self.cap > 4 {
            self.resize(self.cap / 2);
        }

        Some(elem)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let elem = unsafe {
            self.ptr.offset((self.back - 1) % self.cap).read()
        };

        self.back = (self.back - 1) % self.cap;
        self.len -= 1;

        if self.cap > self.len * 2 && self.cap > 4 {
            self.resize(self.cap / 2);
        }

        Some(elem)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    fn resize(&mut self, new_cap: isize) {
        let ptr = unsafe {
            alloc::alloc(Self::layout(new_cap)) as *mut T
        };

        for write_at in 0..self.cap {
            let read_at = (self.front + write_at) % self.cap;
            unsafe {
                ptr.offset(write_at).write(self.ptr.offset(read_at).read());
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

    fn layout(cap: isize) -> Layout {
        Layout::array::<T>(cap as usize).unwrap()
    }
}


impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, Self::layout(self.cap));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::deque::Deque;

    #[test]
    fn basics() {
        let mut queue = Deque::new();

        // Check empty queue behaves right
        assert_eq!(queue.pop_front(), None);

        // Populate queue
        queue.push_back(1);
        queue.push_back(2);
        queue.push_back(3);

        // Check normal removal
        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.push_back(4);
        queue.push_back(5);

        // Check normal removal
        assert_eq!(queue.pop_front(), Some(3));
        assert_eq!(queue.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(queue.pop_front(), Some(5));
        assert_eq!(queue.pop_front(), None);

        // Check the exhaustion case fixed the pointer right
        queue.push_back(6);
        queue.push_back(7);

        // Check normal removal
        assert_eq!(queue.pop_front(), Some(6));
        assert_eq!(queue.pop_front(), Some(7));
        assert_eq!(queue.pop_front(), None);

        queue.push_front(1);
        queue.push_front(2);
        queue.push_front(3);
        queue.push_front(4);
        queue.push_front(5);
        queue.push_front(6);
        queue.push_front(7);
        queue.push_front(8);

        assert_eq!(queue.len(), 8);

        assert_eq!(queue.pop_back(), Some(1));
        assert_eq!(queue.pop_back(), Some(2));
        assert_eq!(queue.pop_back(), Some(3));
        assert_eq!(queue.pop_back(), Some(4));
        assert_eq!(queue.pop_back(), Some(5));
        assert_eq!(queue.pop_back(), Some(6));
        assert_eq!(queue.pop_back(), Some(7));
        assert_eq!(queue.pop_back(), Some(8));
        assert_eq!(queue.pop_back(), None);

        queue.push_front(1);
        queue.push_front(2);
        queue.push_back(3);
        queue.push_back(4);

        assert_eq!(queue.pop_front(), Some(2));
        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_back(), Some(4));
        assert_eq!(queue.pop_back(), Some(3));
        assert_eq!(queue.pop_front(), None);
        assert_eq!(queue.pop_back(), None);
    }
}