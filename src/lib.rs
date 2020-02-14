pub mod node;
pub mod helper;

#[cfg(test)]
mod atom_test {
    use crate::node::Node::*;
    #[test]
    fn atom_cmp() {
        assert_eq!(Atom("ratatouille"), Atom("ratatouille"));
        assert_ne!(Atom("ratatouille"), Atom("baguette"))
    }
}

#[cfg(test)]
mod pair_test {
    use crate::node::Node::*;
    use crate::node::*;
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
        assert_eq!(type_of(TypeNode(Type::Universe(0))), Type::Universe(1));
        assert_eq!(type_of(TypeNode(Type::Universe(1))), Type::Universe(2));
    }
}

#[cfg(test)]
mod judgement {
    use crate::node::Node::*;
    use crate::helper::*;
    #[test]
    fn same_type() {
        assert_eq!(is_same_type(Atom("a"), Atom("b")), true)
    }
}
