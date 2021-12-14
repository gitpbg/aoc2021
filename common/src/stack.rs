pub struct Stack<T> {
    data: Vec<T>,
}

impl<T: Default + Copy> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    pub fn peek(&self) -> Option<T> {
        if self.data.len() > 0 {
            Some(self.data[self.data.len() - 1])
        } else {
            None
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let l = self.data.len();
        if l > 0 {
            let val: T = self.data.remove(l - 1);
            Some(val)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}
