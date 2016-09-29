struct Node {
    data:   i32,
    next:   Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node {
            data: data,
            next: None,
        }
    }

    fn assign_next(&mut self, next: Node) {
        self.next = Some(Box::new(next));
    }
}

fn main() {
    let node        = Node::new(1);
    let second_node = Node::new(2);

    node.assign_next(second_node);
}
