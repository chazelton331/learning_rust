struct Node {
    value: i32,
    next:  Box<Option<Node>>,
}

struct LinkedList {
    head:  Box<Option<Node>>,
    curr:  Box<Option<Node>>,
    count: i32,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node { value: value, next: Box::new(None) }
    }

    //pub fn assign_next(&mut self, next: Node) {

    //}
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: Box::new(None), curr: Box::new(None), count: 0 }
    }

    pub fn add_node(&mut self, node: Node) {
        self.count += 1;
        self.curr   = Box::new(Some(node));

        match self.head {
            Some(Box::new(n)) => {
                // NOOP
            },
            None => {
                self.head = node;
            },
        }
    }
}

fn main() {
    let mut node1 = Node::new(1);
    println!("{}", node1.value);

    let node2 = Node::new(2);
    println!("{}", node2.value);

    node1.next = Box::new(Some(node2));

    println!("{}", node1.next.unwrap().value);
}
