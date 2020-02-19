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
    Atom,
    Pair(Pair<Type>),
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
        Node::Atom(_) => Type::Atom,
        Node::PairNode(p) => Type::Pair(Pair(Box::new(type_of(*p.0)), Box::new(type_of(*p.1)))),
        Node::TypeNode(Type::Universe(n)) => Type::Universe(n + 1),
        Node::TypeNode(_) => Type::Universe(0),
        Node::Unit => Type::Unit,
    }
}

#[cfg(test)]
mod test {
    use crate::node::Node::*;
    use crate::node::*;

    #[test]
    fn atom_cmp() {
        assert_eq!(Atom("ratatouille"), Atom("ratatouille"));
        assert_ne!(Atom("ratatouille"), Atom("baguette"))
    }

    #[test]
    fn pair_cmp() {
        assert_eq!(
            PairNode(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("ratatouille")),
            )),
            PairNode(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("ratatouille")),
            ))
        );

        assert_ne!(
            PairNode(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("ratatouille"))
            )),
            PairNode(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("baguette")),
            ))
        );

        assert_ne!(
            PairNode(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("ratatouille")),
            )),
            PairNode(Pair(
                Box::new(Atom("baguette")),
                Box::new(Atom("ratatouille")),
            ))
        );

        assert_ne!(
            PairNode(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("baguette")),
            )),
            PairNode(Pair(Box::new(Atom("baguette")), Box::new(Atom("baguette")),))
        );

        assert_ne!(
            PairNode(Pair(Box::new(Atom("baguette")), Box::new(Atom("baguette")),)),
            PairNode(Pair(
                Box::new(Atom("baguette")),
                Box::new(Atom("ratatouille")),
            ))
        );
    }

    #[test]
    fn pair_cons() {
        assert_eq!(
            Pair::cons(Atom("ratatouille"), Atom("baguette")),
            Pair(Box::new(Atom("ratatouille")), Box::new(Atom("baguette")),)
        )
    }

    #[test]
    fn pair_car() {
        assert_eq!(
            Pair::car(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("baguette"))
            )),
            Atom("ratatouille")
        );
    }

    #[test]
    fn pair_cdr() {
        assert_eq!(
            Pair::cdr(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Atom("baguette")),
            )),
            Atom("baguette")
        );

        assert_eq!(
            Pair::cdr(Pair(
                Box::new(Atom("ratatouille")),
                Box::new(Node::PairNode(Pair(
                    Box::new(Atom("baguette")),
                    Box::new(Atom("aubergine")),
                ))),
            )),
            Node::PairNode(Pair(
                Box::new(Atom("baguette")),
                Box::new(Atom("aubergine")),
            ))
        )
    }

    #[test]
    fn type_of_term() {
        use crate::node::type_of;
        use crate::node::Type;
        assert_eq!(type_of(Atom("yo")), Type::Atom);
    }
}
