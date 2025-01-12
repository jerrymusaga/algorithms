use std::collections::VecDeque;



#[derive(Debug)]
struct MyNode<T>{
    value: T,
    right: Option<Box<MyNode<T>>>,
    left: Option<Box<MyNode<T>>>
}

impl<T> MyNode<T>{
    fn new(value: T) -> Self {
        MyNode {
            value,
            right: None,
            left: None
        }
    }
}

#[derive(Debug)]
struct MyBinaryTree<T>{
    root: Option<Box<MyNode<T>>>
}

impl<T:std::fmt::Debug> MyBinaryTree<T>{
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn insert_new_node(&mut self, value: T){
        let create_new_node = Box::new(MyNode::new(value));
        if self.root.is_none(){
            self.root = Some(create_new_node);
        }
    }

    fn breadth_first(&mut self){
        if let Some(root) = &mut self.root {
            let mut queue = VecDeque::new();
            queue.push_back(&**root);

            while !queue.is_empty(){
                if let Some(current) = queue.pop_front(){
                    print!("current value: {:?}\n", current.value);
                    if let Some(left) = &current.left {
                        queue.push_back(left);
                        // println!("left current value: {:?}", left);
                    }
                    if let Some(right) = &current.right {
                        queue.push_back(right);
                        // println!("right current value: {:?}", right);
                    }
                }

            }
        }
    }

    fn dfs(&self) {
        Self::dfs_recursive(&self.root);
        
    }

    fn dfs_recursive(my_node: &Option<Box<MyNode<T>>>) {
        if let Some(current) = my_node {
            Self::dfs_recursive(&current.left);
            println!("current value {:?}", current.value);
            Self::dfs_recursive(&current.right);
        }
    }
}
fn run() {
    let mut tree = MyBinaryTree::new();
    tree.insert_new_node(10);

    if let Some(root) = &mut tree.root {
        root.left = Some(Box::new(MyNode::new(3)));
        root.right = Some(Box::new(MyNode::new(4)));

        if let Some(left) = &mut root.left {
            left.left = Some(Box::new(MyNode::new(5)));
            left.right = Some(Box::new(MyNode::new(6)));
        }
    }

    println!("");
    println!("\nDisplay of the Binary tree\n {:?}", &tree);
    println!("");

    println!("\nBFS");
    tree.breadth_first();

    println!("\nDFS");
    tree.dfs();
}