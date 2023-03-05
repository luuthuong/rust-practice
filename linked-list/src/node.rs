pub struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>
}

pub trait NodeService<T>{
    fn new(data: T) -> Self<T>;
    fn append(&mut self, data:T);
    fn print(&self);
}
impl NodeService<T> for Node<T>{
    fn new(data: T) -> Self <T> {
        Node{
            data,
            next: None
        }
    }

    fn append(&mut self, data: T) {
        match &mut self.next {
            Some(node) => node.append(data),
            None => self.next = Some(Box::new(Node::new(data)))
        }
    }

    fn print(&self) {
        let mut node = self;
        loop {
            println!("{:?}", node.data);
            match &node.next {
                Some(_node) => node = _node,
                None => break
            }
        }
    }
}