//! Linked List implementation

use std::mem;

/// Linked List
pub struct LList<T> {
    size: usize,
    head: LCell<T>,
}

/// Cell containing data or nothing (end of list)
enum LCell<T> {
    Cell(T, Box<LCell<T>>),
    Nil,
}

impl <T> LList<T> {
    /// new empty list
    pub fn new() -> LList<T> {
        LList { size: 0, head: LCell::Nil}
    }

    /// is the list empty
    pub fn is_empty(&self) -> bool {
        self.size==0
    }

    /// list size (O(1))
    pub fn size(&self) -> usize {
        self.size
    }

    /// push an element to the head of the list
    pub fn push(&mut self, d: T) {
        self.size = self.size+1;
        // detach head
        let lc = mem::replace(&mut self.head,LCell::Nil);
        // replace with new head
        mem::replace(&mut self.head, LCell::Cell(d,Box::new(lc)));
    }

    /// pop an element from the head of the list
    /// panics if the list is empty
    pub fn pop(&mut self) -> T {
        // detach head
        let lc = mem::replace(&mut self.head,LCell::Nil);
        let (d,lcn) = match lc {
            LCell::Nil => {
                mem::replace(&mut self.head,lc);
                panic!("Empty list at pop")
            },
            LCell::Cell(data,next) => (data,*next)
        };
        // replace with new head
        mem::replace(&mut self.head,lcn);
        self.size=self.size-1;
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop(){
        let mut l = LList::new();
        assert_eq!(0,l.size());
        assert_eq!(true,l.is_empty());
        l.push(5);
        assert_eq!(1,l.size());
        assert_eq!(false,l.is_empty());
        assert_eq!(5,l.pop());
        assert_eq!(0,l.size());
        assert_eq!(true,l.is_empty());

        l.push(5);
        l.push(10);
        assert_eq!(2,l.size());
        assert_eq!(false,l.is_empty());
        assert_eq!(10,l.pop());
        assert_eq!(1,l.size());
        assert_eq!(false,l.is_empty());
        assert_eq!(5,l.pop());
        assert_eq!(0,l.size());
        assert_eq!(true,l.is_empty());
    }

    #[test]
    #[should_panic(expected = "Empty list at pop")]
    fn test_empty_push(){
        let mut l:LList<i32> = LList::new();
        assert_eq!(0,l.size());
        assert_eq!(true,l.is_empty());
        l.pop();
    }
}