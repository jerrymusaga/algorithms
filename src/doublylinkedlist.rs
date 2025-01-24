use std::{cell::{RefCell}, rc::Rc};

struct Node{
    element: i32,
    next: Pointer,
    prev: Pointer
}

struct DoublyLinkedList {
    head: Pointer,
    tail: Pointer
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn add(&mut self, element: i32){
        let new_head = Some(Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None
        })));
        // match self.head.take() {
        //     Some(old_head) => {
        //         old_head.borrow_mut().prev = Some(new_head.clone());
        //         new_head.borrow_mut().next = Some(old_head.clone());
        //     }
        // }
    }
}

type Pointer = Option<Rc<RefCell<Node>>>;