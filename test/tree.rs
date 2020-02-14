struct Tree {
    nodes: Vec<Node>,
    root: NodeId,
}

impl Tree {
    fn new(val: i32) -> Tree {
        Tree {
            nodes: vec![Node::new(val)],
            root: 0,
        }
    }

    fn add_node(&mut self, val: i32) {
        let mut n = self.root;
        loop {
            if self.nodes[n].value > val {
                match self.nodes[n].left {
                    Some(k) => n = k,
                    None => {
                        self.nodes.push(Node::new(val));                        
                        self.nodes[n].left = Some(self.nodes.len() - 1);
                        return;
                    },
                }
            } else {
                match self.nodes[n].right {
                    Some(k) => n = k,
                    None => {
                        self.nodes.push(Node::new(val));                        
                        self.nodes[n].right = Some(self.nodes.len());
                        return;
                    },
                }

            }
        }
    }

    fn has_value(&self, val: i32) -> bool {
        let mut n = self.root;
        loop {
            if self.nodes[n].value > val {
                match self.nodes[n].left {
                    Some(k) if self.nodes[k].value == val => return true,
                    Some(k) => n = k,
                    None => return false,
                }
            } else {
                match self.nodes[n].right {
                    Some(k) if self.nodes[k].value == val => return true,
                    Some(k) => n = k,
                    None => return false,
                }

            }
        }
    }
}

struct Node {
    left: Option<NodeId>,
    right: Option<NodeId>,
    value: i32,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            left: None,
            right: None,
            value: val,
        }
    }
}
