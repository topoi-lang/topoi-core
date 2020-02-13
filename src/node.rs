#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Atom(Atom),
    List(List),
}

pub type Atom = &'static str;

// =====================================
// List
// =====================================
pub type List = Vec<Node>;

pub fn cons(list: List, atom: Atom) -> Node {
    let mut cloned_list = list.clone();
    cloned_list.push(Node::Atom(atom));
    Node::List(cloned_list)
}

pub fn car(list: List) -> Node {
    list.first().unwrap().clone()
}

pub fn cdr(list: List) -> Node {
    let mut cloned_list = list.clone();
    cloned_list.remove(0);
    Node::List(cloned_list)
}
