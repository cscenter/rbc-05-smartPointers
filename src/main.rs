#![deny(warnings)]

use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

fn main() {
    let zero_parent: Option<Weak<RefCell<Node>>> = None;
    let root: Rc<RefCell<Node>> = Node::create(String::from("Root"), zero_parent);

    {
        let mut root_ref = RefCell::borrow_mut(root.borrow());

        let weak_root: Option<Weak<RefCell<Node>>> = my_get_weak(&root);
        root_ref.add_left(weak_root, String::from("Left"));
        root_ref.add_right(my_get_weak(&root), String::from("Right"));

        {
            let mut left_node: RefMut<Node> = root_ref.unwrap_left_mut();
            left_node.add_left(my_get_weak(&root), String::from("Leaf"));
        }

        let mut values: Vec<String> = vec![]; // fill this collection with clones: node.value.clone()
        root_ref.traverse(&mut values); // use recursive algorithm
        assert_eq!(vec![String::from("Root"), String::from("Left"), String::from("Leaf"), String::from("Right")],
                   values);
    }


    my_destroy_all(root);
}

struct Node {
    value: String,
    #[allow(dead_code)]
    parent: // weak reference to parent
    left: // strong reference
    right: // strong reference
}

fn my_get_weak(rc: &Rc<RefCell<Node>>) -> Option<Weak<RefCell<Node>>> {

}

