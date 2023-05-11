struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue{
            in_stack: vec![],
            out_stack: vec![]
        }
    }
    
    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.out_stack.len() == 0 {
            while !self.in_stack.is_empty() {
                self.out_stack.push(self.in_stack.pop().unwrap());
            }
        }
        self.out_stack.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.out_stack.len() == 0 {
            while !self.in_stack.is_empty() {
                self.out_stack.push(self.in_stack.pop().unwrap());
            }
        }
        self.out_stack[self.out_stack.len() - 1]
    }
    
    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
