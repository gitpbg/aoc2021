pub mod colorterm;
pub mod stack;
pub mod twodarray;

#[cfg(test)]
mod tests {
    use crate::stack::Stack;
    #[test]
    fn it_works() {
        let mut s: Stack<i32> = Stack::new();
        s.push(10);
        s.push(20);
        assert_eq!(20, s.pop().unwrap());
        assert_eq!(10, s.pop().unwrap());
    }
}
