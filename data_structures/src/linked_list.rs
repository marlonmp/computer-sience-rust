enum NodeAddress<T> {
    None,
    Some(Box<Node<T>>)
}

struct Node<T> {
    value: T,
    next: NodeAddress<T>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T>{
        Node{
            value,
            next: NodeAddress::None,
        }
    }

    pub fn add_next(&mut self, node: Box<Node<T>>) {
        match self.next {
            NodeAddress::Some(ref mut next_node) => next_node.add_next(node),
            NodeAddress::None => self.next = NodeAddress::Some(node),
        }
    }

    pub fn value(self) -> T {
        self.value
    }
}

struct LinkedList<T> {
    len: usize,
    head: NodeAddress<T>
}

impl<T> LinkedList<T> {
    pub fn new(values: &[T]) -> LinkedList<T> {
        LinkedList{
            len: 0,
            head: NodeAddress::None
        }
    }

    pub fn new_empty() -> LinkedList<T> {
        LinkedList{
            len: 0,
            head: NodeAddress::None
        }
    }

    pub fn len(self) -> usize {
        self.len
    }

    fn push_node(&mut self, box_node: Box<Node<T>>) {
        match self.head {
            NodeAddress::Some(ref mut node) => node.add_next(box_node),
            NodeAddress::None => self.head = NodeAddress::Some(box_node)
        }
    }

    pub fn push(&mut self, new_value: T) {
        let node = Node::new(new_value);
        let box_node = Box::new(node);

        self.push_node(box_node);

        self.len += 1;
    }

    pub fn append_array(&mut self, array: &[T]) {
        let mut head: NodeAddress<T> = NodeAddress::None;

        for num in array {
            let new_node = Node::new(num);
            let box_node = Box::new(new_node);

            if let NodeAddress::Some(ref mut h) = head {
                h.add_next(box_node);
                continue;
            }

            head = NodeAddress::Some(box_node);
        }

        if let NodeAddress::Some(node) = head {
            self.push_node(node)
        }

        self.len = array.len();
    }
}

pub fn linked_list() {
    println!("\nlinked list");

    let mut list:LinkedList<i32> = LinkedList::new(&[1, 2, 3, 4]);

    list.push(1);

    println!("len: {}", list.len());
}
