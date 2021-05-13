use std::alloc::{self, Layout};

pub struct Stack<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        let layout = Layout::array::<T>(4).unwrap();
        Stack {
            ptr: unsafe { alloc::alloc(layout) as *mut T },
            cap: 4,
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        self.grow_if_no_space_left();
        unsafe { self.ptr.add(self.len).write(elem) }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.shrink_if_too_much_space();
        let elem = Some(unsafe { self.ptr.add(self.len - 1).read() });
        self.len -= 1;
        elem
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        unsafe { self.ptr.add(self.len - 1).as_ref() }
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        unsafe { self.ptr.add(self.len - 1).as_mut() }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn grow_if_no_space_left(&mut self) {
        if self.cap == self.len {
            let new_cap = self.cap * 2;
            self.resize(new_cap);
        }
    }

    fn shrink_if_too_much_space(&mut self) {
        if self.cap > 4 && self.cap > self.len * 2 {
            let new_cap = self.cap / 2;
            self.resize(new_cap);
            self.cap = new_cap;
        }
    }

    fn resize(&mut self, new_cap: usize) {
        unsafe {
            let layout = Layout::array::<T>(self.cap).unwrap();
            self.ptr = alloc::realloc(self.ptr as *mut u8, layout, new_cap) as *mut T;
        }
        self.cap = new_cap;
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        self.len = 0;
        unsafe {
            let layout = Layout::array::<T>(self.cap).unwrap();
            self.cap = 0;
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        // Check empty stack behaves right
        assert_eq!(stack.pop(), None);

        // Populate stack
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        stack.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }
}