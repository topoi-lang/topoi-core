pub mod node;

#[cfg(test)]
mod tests {
    use crate::node::*;
    use crate::node::Node::*;

    #[test]
    fn atom_cmp() {
        assert_eq!(Atom("ratatouille"), Atom("ratatouille"));
        assert_ne!(Atom("ratatouille"), Atom("baguette"))
    }

    #[test]
    fn list_cmp() {
        assert_eq!(
            List(vec![Atom("ratatouille")]),
            List(vec![Atom("ratatouille")])
        );

        assert_ne!(
            List(vec![Atom("ratatouille")]),
            List(vec![Atom("baguette")])
        );
    }

    #[test]
    fn list_cons() {
        use crate::node::cons;
        assert_eq!(
            cons(vec![Atom("ratatouille")], "baguette"),
            List(vec![Atom("ratatouille"), Atom("baguette")])
        )
    }

    #[test]
    fn list_car() {
        use crate::node::car;
        assert_eq!(
            car(vec![Atom("ratatouille"), Atom("baguette")]),
            Atom("ratatouille")
        );
    }

    #[test]
    fn list_cdr() {
        use crate::node::cdr;
        assert_eq!(
            cdr(vec![Atom("ratatouille"), Atom("baguette")]),
            List(vec![Atom("baguette")])
        );

        assert_eq!(
            cdr(vec![
                Atom("ratatouille"),
                Atom("baguette"),
                Atom("aubergine")
            ]),
            List(vec![Atom("baguette"), Atom("aubergine")])
        )
    }

    #[test]
    fn type_of_term() {
        assert_eq!(type_of(Atom("yo")), Type(0));
        assert_eq!(type_of(List(vec![])), Type(0));
        assert_eq!(type_of(Type(0)), Type(1))
    }
}
