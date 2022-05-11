use crate::lexer;
use lexer::{Token, TokenType};
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

// MemoryEntry but with span, with a name (RAM, RAM2, FLASH)
// attribute for ex: "(rx)"
// struct MemoryEntry {
//     line: usize,
//     origin: u64,
//     length: u64,
// }
// Look ahead

// no parent,
// slice of tokens --> MemoryEntry datastructure
// tokens do not need to have tree datastructure
// check previous token or next token
#[derive(Clone, Debug)]
struct Node {
    token: Token,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_leaf(token: Token) -> Self {
        Node {
            token,
            parent: None,
            children: Vec::new(),
        }
    }
    fn new_tree(token: Token, children: Vec<Rc<RefCell<Node>>>) -> Self {
        Node {
            token,
            parent: None,
            children,
        }
    }
}
type TokenIt<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

fn parse_unit(mut it: TokenIt) -> Option<Rc<RefCell<Node>>> {
    if let Some(token) = it.peek() {
        match &token.token_type {
            TokenType::Word(w) if *w == "K".to_string() => {
                return Some(Rc::new(RefCell::new(Node::new_leaf(
                    it.next().unwrap().clone(),
                ))))
            }
            TokenType::Word(w) if *w == "M".to_string() => {
                return Some(Rc::new(RefCell::new(Node::new_leaf(
                    it.next().unwrap().clone(),
                ))))
            }
            // Error
            _ => (),
        };
    }
    None
}
fn parse_number(mut it: TokenIt) -> Rc<RefCell<Node>> {
    let token = it.next().unwrap().clone();
    if let Some(unit) = parse_unit(it) {
        let temp = Rc::new(RefCell::new(Node::new_tree(token, vec![unit])));
        temp.borrow().children[0].borrow_mut().parent = Some(Rc::downgrade(&temp));
        // temp.children.push(unit);
        return temp;
    }
    Rc::new(RefCell::new(Node::new_leaf(token)))
}

fn parse_expression() {}

fn parse(tokens: Vec<Token>) {
    let mut it = tokens.iter().peekable();
    //parse_number(it);
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, ops::Deref};

    use crate::lexer::lexer;

    use super::*;

    #[test]
    fn parser_number_1() {
        let vec_num = lexer("42");
        let node = parse_number(vec_num.iter().peekable());
        assert!(node.borrow_mut().children.is_empty());
        assert!(node.borrow_mut().parent.is_none());
        assert_eq!(node.borrow_mut().token.token_type, TokenType::Number(42));
    }
    #[test]
    fn parser_number_2() {
        let vec_num = lexer("42K");
        let node = parse_number(vec_num.iter().peekable());
        assert_eq!(
            node.borrow_mut().children[0].borrow_mut().token.token_type,
            TokenType::Word("K".to_string()),
        );

        println!(
            "PAR: {:#?}",
            node.borrow_mut().children[0]
                //.borrow()
                .parent
                .as_ref()
                .unwrap()
                .borrow()
        );
        // assert_eq!(
        //     node.borrow_mut().children[0].borrow_mut().token.token_type,
        //     TokenType::Word("K".to_string()),
        // );
    }

    #[test]
    fn parser_unit_1() {
        let vec_unit = lexer("K");
        let node = parse_unit(vec_unit.iter().peekable());
        //assert_eq!(node.unwrap().token.token_type, TokenType::Word("K"));
    }

    #[test]
    fn parser_unit_2() {
        let vec_unit = lexer("");
        let node = parse_unit(vec_unit.iter().peekable());
        //assert_eq!(node, None);
    }

    #[test]
    fn parser_unit_3() {
        let vec_unit = lexer("shoot");
        let node = parse_unit(vec_unit.iter().peekable());
        //assert_eq!(node, None);
    }
}
