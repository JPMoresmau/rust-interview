//! Linked List implementation

use std::mem;
use std::iter::IntoIterator;
use std::iter::FromIterator;

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

impl <'a, T> LList<T> {
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

    /// head
    pub fn head(&self) -> &T {
        match &self.head {
            LCell::Nil => panic!("Empty list at head"),
            LCell::Cell(data,_) => data
        }
    }

    /// reverse in place
    pub fn reverse(&mut self) {
        let mut rl = LList::new();
        while !self.is_empty(){
            rl.push(self.pop());
        }
        mem::replace(&mut self.head,rl.head);
        self.size=rl.size;
    }

    /// remove all content
    pub fn clear(&mut self) {
        mem::replace(&mut self.head,LCell::Nil);
        self.size=0;
    }

    /// map content to new values via a function
    pub fn map<F>(&mut self, f: F) where F: Fn(&T) -> T {
        let mut cell = &mut self.head;
        while let LCell::Cell(d,n) = cell {
            mem::replace(d, f(d));
            cell = n;
        }
    }
}

/// convert into an iterator
impl <'a, T> IntoIterator for &'a LList<T> {
    type Item = &'a T;
    type IntoIter = LLIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LLIterator{head: &self.head}
    }
}

/// an iterator holding a reference to the cell of the list
pub struct LLIterator<'a,T> {
  head: &'a LCell<T>
}

/// iterator implementation
impl <'a, T> Iterator for LLIterator<'a, T> {
   type Item = &'a T;

   fn next(&mut self) -> Option<Self::Item> {
        let lc = mem::replace(&mut self.head,&LCell::Nil);
        let oc = match lc {
           LCell::Nil => None,
           LCell::Cell(data,next) => Some((data,next))
        };
        oc.map(|(d,n)| {
                mem::replace(&mut self.head,&(*n));
                d
            })
   }
     
} 

/// convert from an iterator
impl <T> FromIterator<T> for LList<T> {
    
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut l = LList::new();
        for i in iter {
            l.push(i);
        }
        l.reverse();
        l
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
    fn test_empty_pop(){
        let mut l:LList<i32> = LList::new();
        assert_eq!(0,l.size());
        assert_eq!(true,l.is_empty());
        l.pop();
    }

    #[test]
    #[should_panic(expected = "Empty list at pop")]
    fn test_empty_pop_after_clear(){
        let mut l = LList::new();
        l.push(5);
        l.clear();
        l.pop();
    }

    #[test]
    fn test_head(){
        let mut l = LList::new();
        l.push(5);
        assert_eq!(&5,l.head());
        assert_eq!(&5,l.head());
        l.push(10);
        assert_eq!(&10,l.head());
        assert_eq!(10,l.pop());
        assert_eq!(&5,l.head());
    }

    #[test]
    #[should_panic(expected = "Empty list at head")]
    fn test_empty_head(){
        let l:LList<i32> = LList::new();
        l.head();
    }

    #[test]
    fn test_reverse(){
        let mut l = LList::new();
        l.push(5);
        l.push(10);
        l.push(20);
        assert_eq!(3,l.size());
        l.reverse();
        assert_eq!(3,l.size());
        assert_eq!(5,l.pop());
        assert_eq!(10,l.pop());
        assert_eq!(20,l.pop());
        assert_eq!(0,l.size());
    }

    #[test]
    fn test_into_iter(){
        let mut l = LList::new();
        l.push(5);
        l.push(10);
        l.push(20);
        assert_eq!(3,l.size());
        let mut v=vec!();
        for d in l.into_iter() {
            v.push(d.clone());
        }
        assert_eq!(3,l.size());
        assert_eq!(vec!(20,10,5),v);
    }

    #[test]
    fn test_from_iter(){
        let iter = (0..3).into_iter();
        let mut l = LList::from_iter(iter);
        assert_eq!(3,l.size());
        assert_eq!(0,l.pop());
        assert_eq!(1,l.pop());
        assert_eq!(2,l.pop());
    }

    #[test]
    fn test_clear(){
        let mut l = LList::new();
        l.push(5);
        l.push(10);
        l.push(20);
        assert_eq!(3,l.size());
        l.clear();
        assert_eq!(0,l.size());
    }

    #[test]
    fn test_map(){
        let mut l = LList::new();
        l.push(5);
        l.push(10);
        l.push(20);
        l.map(|i| i * 3);
        assert_eq!(60,l.pop());
        assert_eq!(30,l.pop());
        assert_eq!(15,l.pop());
    }
}