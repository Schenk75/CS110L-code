use std::fmt;

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl Node {
    fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node {value, next}
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {head: None, size: 0}
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    // pub fn display(&self) {
    //     let mut current = &self.head;
    //     let mut result = String::new();
        
    //     loop {
    //         match current {
    //             Some(node) => {
    //                 if result.is_empty() {
    //                     result = format!("{}", node.value); 
    //                 } else {
    //                    result = format!("{}->{}", result, node.value); 
    //                 }
    //                 current = &node.next;
    //             },
    //             None => break,
    //         }
    //     }
    //     println!("{}", result);
    // }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut current = &self.head;
        let mut result = String::new();
        
        loop {
            match current {
                Some(node) => {
                    if result.is_empty() {
                        result = format!("{}", node.value); 
                    } else {
                       result = format!("{}->{}", result, node.value); 
                    }
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::new();
    // linked_list.display();
    println!("{}", linked_list);
    println!("Size: {}", linked_list.get_size());
    println!("Is empty? {}\n", linked_list.is_empty());

    linked_list.push(1);
    linked_list.push(2);
    linked_list.push(3);
    linked_list.push(4);
    // linked_list.display();
    println!("{}", linked_list);
    println!("Size: {}", linked_list.get_size());
    println!("Is empty? {}\n", linked_list.is_empty());

    let elem = linked_list.pop();
    // linked_list.display();
    println!("{}", linked_list);
    println!("Size: {}", linked_list.get_size());
    println!("Is empty? {}", linked_list.is_empty());
    match elem {
        Some(x) => println!("Pop element: {}", x),
        None => println!("Pop nothing"),
    }
}