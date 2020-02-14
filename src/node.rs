#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Atom(&'static str),
    PairNode(Pair<Node>),
    TypeNode(Type),
    Unit,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Universe(i64),
    AtomTy,
    PairTy(Pair<Type>),
    Unit,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Pair<T>(pub Box<T>, pub Box<T>);

impl<T> Pair<T> {
    pub fn cons(node1: T, node2: T) -> Pair<T> {
        Pair(Box::new(node1), Box::new(node2))
    }

    pub fn car(self) -> T {
        *self.0
    }

    pub fn cdr(self) -> T {
        *self.1
    }
}

pub fn type_of(node: Node) -> Type {
    match node {
        Node::Atom(_) => Type::AtomTy,
        Node::PairNode(p) => Type::PairTy(Pair(Box::new(type_of(*p.0)), Box::new(type_of(*p.1)))),
        Node::TypeNode(Type::Universe(n)) => Type::Universe(n + 1),
        Node::TypeNode(_) => Type::Universe(0),
        Node::Unit => Type::Unit,
    }
}
