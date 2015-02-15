#![feature(box_syntax)]

// Allow Cons and Nil to be referred to without namespacing
use List::{Cons, Nil};

// A linked list node, which can take on any of these two variants
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, box self)
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
      match *self {
            Cons(head, ref tail) => {
                let mut acc = "[".to_string();
                acc.push_str(&format!("{}", head));
                tail.do_stringify(acc)
            },
            Nil => {
                "".to_string()
            }
        }

    }
    fn do_stringify(&self, mut acc: String) -> String {
        match *self {
            Cons(head, ref tail) => {
                acc.push_str(", ");
                acc.push_str(&format!("{}", head));
                tail.do_stringify(acc)
            },
            Nil => {
                acc.push_str("]");
                acc
            },
        }
      
    }

}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}