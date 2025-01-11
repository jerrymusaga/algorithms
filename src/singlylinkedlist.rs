#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer
}

#[derive(Debug)]
struct LinkedList {
    head: Pointer
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn add(&mut self, element: i32) {
    
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None
        //         }));
        //         self.head = new_node;
        //     },
        //     Some(prev_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(prev_head)
        //         }));
        //         self.head = new_node;
        //     }

        // }

        let prev_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: prev_head
        }));
        self.head = new_head;
        
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take(){
            Some(prev_head) => {
                self.head = prev_head.next;
                Some(prev_head.element)
            },
            None => None
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none(){
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }

}

type Pointer = Option<Box<Node>>;

pub fn run() {
    let mut list = LinkedList::new();
    list.add(5);
    list.add(10);
    list.add(20);
    list.add(30);
    list.add(50);
    list.remove();

    // println!("List: {:?} ", list);

    list.print();


}