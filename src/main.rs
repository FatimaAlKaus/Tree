use std::cmp::Ord;
use rand;
use std::fmt::Display;

struct Node<T>
    where T:
    Default +
    Ord +
    Display + {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
    where T:
    Default +
    Ord +
    Display + {
    fn new(data: T) -> Node<T> {
        Node { data, left: None, right: None }
    }
    fn print(&self) {
        if let Some(left) = &self.left {
            left.print();
        }
        println!("{}", self.data);
        if let Some(right) = &self.right {
            right.print();
        }
    }
    fn add(&mut self, data: T) {
        let mut pointer = self;
        loop {
            if data >= pointer.data {
                if let Some(ref mut right) = pointer.right {
                    pointer = right;
                } else {
                    pointer.right = Some(Box::new(Node::new(data)));
                    break;
                }
            } else {
                if let Some(ref mut left) = pointer.left {
                    pointer = left;
                } else {
                    pointer.left = Some(Box::new(Node::new(data)));
                    break;
                }
            }
        }
    }
}


fn main() {
    let mut tree = Node::new(10);
    for _ in 0..100 {
        tree.add(rand::random::<i32>() % 100);
    }
    tree.print();
}
