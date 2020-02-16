pub mod node;

#[cfg(test)]
mod tests {
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
