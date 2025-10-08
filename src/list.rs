#[allow(unused_imports)]
use std::{fmt::Display, mem};

#[derive(Debug)]
pub enum ListNode<T> {
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
  // Use the implementation of this method to guide your implementation of
  // `insert` and `reverse`
  /// Deletes a node from the list
  pub fn delete(&mut self) {
    // Temporarily replaces the current node with default value (Nil).
    // In exchange, we get to take ownership of the current node instead of just
    // having it by mutable reference.
    let as_owned: ListNode<T> = mem::take(self);
    match as_owned {
      ListNode::Nil => {}
      ListNode::Cons(_, next) => {
        // Write the next node to the current node
        *self = *next;
      }
    }
  }
}

// Required methods for `ListNode<T>`
impl<T> ListNode<T> {
    /// Creates a new empty list
    pub fn new() -> Self {
        ListNode::Nil
    }
    /// Inserts a new list node with value `value` after `self` and returns a reference to the new
    /// node
    pub fn insert(&mut self, value: T) -> &mut Self {
      let as_owned: ListNode<T> = mem::take(self);
      match as_owned {
        ListNode::Nil => {
          *self = ListNode::Cons(value, Box::new(ListNode::Nil));
          self
        }
        ListNode::Cons(self_val, mut next) => {
          // Write the next node to the new node
          let next_as_owned: ListNode<T> = mem::take(&mut *next);
          let new_node: ListNode<T> = ListNode::Cons(value, Box::new(next_as_owned));
          *self = ListNode::Cons(self_val, Box::new(new_node));
          match self {
            ListNode::Cons(_, ref mut next) => &mut **next,
            ListNode::Nil => unreachable!(),
          }
        }
      }
    }
  
    /// Reverses the list in place. Adapted from C algorithm 
    pub fn reverse(&mut self) {
      //prev initialized as Nil so that head will point to Nil (making it the new tail)
      let mut prev : ListNode<T> = ListNode::Nil;
      //take ownership of current node
      let mut current : ListNode<T> = mem::take(self);
      //empty "pointer" for next
      let mut next : ListNode<T>;
      loop {
        match current {
          //If Nil, we have reached the end of list
          ListNode::Nil => break,
          //Else we need to reverse and shift "pointers"
          ListNode::Cons(value, nxt) => {
            //save what was after current as next
            next = *nxt;
            //then reverse edge so now current points to whatever originally came before it
            let current_reversed = ListNode::Cons(value, Box::new(prev));
            //shift "pointers"
            prev = current_reversed;
            current = next;
          }
        }
      }
      //If current is Nil, previous is the new head of the list, update self
      *self = prev;
  }
}

// let mut current: &mut ListNode<T> = self;
//         let mut next : &mut ListNode<T>;
//         match current {
//           ListNode::Nil => return,
//           ListNode::Cons(_, ref mut nxt) => {
//             current = &mut **nxt;
//             *nxt = Box::new(ListNode::new());
//           }
//         }
//         let mut previous : &mut ListNode<T>;
//         current = next;
//         loop {
//           match current {
//             ListNode::Nil =>  {},
//             ListNode::Cons(_, nxt) => {
              
//             }
//           }
//         }
// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        ListNode::Nil
    }
}

// Implement `PartialEq` for `ListNode<T>`
impl<T> PartialEq for ListNode<T> where T: PartialEq{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
          // Nil equals Nil
          (ListNode::Nil, ListNode::Nil) => true,
          // If both cons, lists are equal iff head values are equal and rest of list is equal
          (ListNode::Cons(val1, next1), ListNode::Cons(val2, next2)) => {
            *val1 == *val2 && next1.eq(next2)
          },
          // O/w types are mismatched so not equal
          _ => false
        }
    }
}

// Implement `Eq` for `ListNode<T>`
impl<T> Eq for ListNode<T> where T: Eq{}

// Implement `Display` for `ListNode<T>`
impl<T> Display for ListNode<T> where T: Display{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        // Write Nil if Nil encountered
        ListNode::Nil => write!(f, "Nil"),
        // O/w Display value, write an arrow, and display rest of list with recursive call
        // Potential errors propagated with ? operator
        ListNode::Cons(val1, next1) => {
          val1.fmt(f)?;
          write!(f, " -> ")?;
          next1.fmt(f)
        }
      }
  }
}

// Implement `From<Vec<T>>` for `ListNode<T>`
impl<T> From<Vec<T>> for ListNode<T> {
  fn from(vec: Vec<T>) -> Self {
    let mut head: ListNode<T> = ListNode::default();
    let mut list: &mut ListNode<T> = &mut head;
    for x in vec.into_iter() {
      list = list.insert(x);
    }
    head
  }
}

// Implement `From<ListNode<T>>` for `Vec<T>`
impl<T> From<ListNode<T>> for Vec<T> {
  fn from(list: ListNode<T>) -> Self {
    let mut vec: Vec<T> = Vec::new();
    let mut current : ListNode<T> = list;
      loop {
        match current {
          ListNode::Nil => break,
          ListNode::Cons(value, next) => {
            vec.push(value);
            current = *next;
          }
        }
      }
      vec
  }
}
