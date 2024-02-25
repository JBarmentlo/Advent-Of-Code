use std::{borrow::BorrowMut, rc::{Rc, Weak}};
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Node {
    name: String,
    children: RefCell<Vec<Node>>,
    parent: Weak<Node>
}

impl Node {
    fn add_child(&mut self, child: Node) {
        self.children.borrow_mut().push(child);
    }

    fn empty(name: &str) -> Node {
        Node {
            name: name.to_string(),
            children: RefCell::new(Vec::new()),
            parent: Weak::new()
        }
    }

    fn new_with_daddy(name: &str, parent: Weak<Node>) -> Node {
        Node {
            name: name.to_string(),
            children: RefCell::new(Vec::new()),
            parent: parent,
        }
    }
}



fn main() {
    println!("Hello, world!");
    let root = Node::empty("root");
    let root = Rc::new(root);
    dbg!(&root);

    let cain = Node::new_with_daddy("cain", Rc::downgrade(&root));
    // root.borrow_mut();
    root.children.borrow_mut().push(cain);

    dbg!(&root);
    dbg!(&cain);

}
