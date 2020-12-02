use std::fmt;
use std::rc::Rc;
 
struct LinkedList {
    head: Option<Rc<Node>>,
    size: usize,
}

struct Node {
    value: u32,
    next: Option<Rc<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Rc<Node>>) -> Node {
        Node {value: value, next: next}
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&self, value: u32) -> LinkedList {
        let new_node: Rc<Node> = Rc::new(Node::new(value, self.head.clone()));
        LinkedList {head: Some(new_node), size: self.size + 1}
    }
    
    // A tuple in Rust is like a struct -- you can access the zeroth element of 
    // a tuple name tup by writing "tup.0", you can access the element at index
    // 2 by writing "tup.2", etc.
    pub fn pop_front(&self) -> (Option<LinkedList>, Option<u32>) {
        match &self.head {
            Some(node) => {
                let val = node.value;
                let new_head: Option<Rc<Node>> = node.next.clone();
                let new_list = LinkedList {head: new_head, size: self.size - 1};
                (Some(new_list), Some(val))
            },
            None => (None, None)
        }
    }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Rc<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

// For sake of simplicity, we removed the impl Drop here. You can still override
// it, you just might have to use some Rust features that are currently unstable.
// If you really wanted it to be efficient, you might just use unsafe rust.

fn main() {
    let list: LinkedList = LinkedList::new();
    let version1 = list.push_front(10);
    let version2 = version1.push_front(5);
    let version3 = version2.push_front(3);
    let version4 = version2.push_front(4);
    let (version5, result) = version4.pop_front();
    println!("version1: {} has size {}", version1, version1.get_size());
    println!("version2: {} has size {}", version2, version2.get_size());
    println!("version3: {} has size {}", version3, version3.get_size());
    println!("version4: {} has size {}", version4, version4.get_size());
    println!("version5: {}, popped value: {}", version5.unwrap(), result.unwrap());
    
    
    
}