use crate::node::Node;
use crate::node::type_of;

pub fn is_same_type(node1: Node, node2: Node) -> bool {
    type_of(node1) == type_of(node2)
}

pub fn is_same(node1: Node, node2: Node) -> bool {
    is_same_type(node1.clone(), node2.clone()) && node1 == node2
}
