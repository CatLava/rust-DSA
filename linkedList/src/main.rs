use std::cell::RefCell;
use std::rc::Rc;

// Since the RC is repeated make a type alias
type SingleLink = Option<Rc<RefCell<Node>>>;
// Because the node size isn't known at compile time
// need RC or boxs
#[derive(Clone)]
struct Node {
    value: String,
    // Ooption is Some or None
    next: SingleLink
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }
}
fn main() {
    println!("Hello, world!");
}
