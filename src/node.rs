#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Atom(&'static str),
    Pair(Box<Node>, Box<Node>),
    Type(Type),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Universe(i64),
    Atom,
    Pair(Box<Type>, Box<Type>)
}

pub fn cons(node: Node, atom: Node) -> Node {
    Node::Pair(Box::new(node), Box::new(atom))
}

pub fn car(node: Node) -> Result<Node, &'static str> {
    match node {
        Node::Pair(a, _) => Result::Ok(*a),
        _ => Result::Err("Can only apply car to Pair")
    }
}

pub fn cdr(node: Node) -> Result<Node, &'static str> {
    match node {
        Node::Pair(_, b) => Result::Ok(*b),
        _ => Result::Err("Cannot apply cdr to Pair")
    }
}

pub fn type_of(node: Node) -> Type {
    match node {
        Node::Atom(_) => Type::Atom,
        Node::Pair(a, b) => Type::Pair(Box::new(type_of(*a)), Box::new(type_of(*b))),
        Node::Type(Type::Universe(n)) => Type::Universe(n + 1),
        Node::Type(_) => Type::Universe(0)
    }
}