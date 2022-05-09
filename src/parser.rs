use crate::lexer;
use lexer::{Token, TokenType};

#[derive(Clone, Debug, PartialEq)]
struct Node<'a> {
    token: Token<'a>,
    parent: Option<Box<Node<'a>>>,
    children: Box<Vec<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new_leaf(token: Token<'a>) -> Self {
        Node {
            token,
            parent: None,
            children: Box::new(Vec::new()),
        }
    }
    fn new_tree(token: Token<'a>, childs: Vec<Node<'a>>) -> Self {
        Node {
            token,
            parent: None,
            children: Box::new(childs),
        }
    }
}
type TokenIt<'a> = std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>;

fn parse_unit(mut it: TokenIt) -> Option<Node> {
    if let Some(token) = it.peek() {
        match token.token_type {
            TokenType::Word("K") | TokenType::Word("M") => {
                return Some(Node::new_leaf(*it.next().unwrap()));
            }
            // Error
            _ => (),
        };
    }
    None
}
fn parse_number(mut it: TokenIt) -> Node {
    let token = *it.next().unwrap();
    if let Some(mut unit) = parse_unit(it) {
        let mut temp = Node::new_tree(token, vec![unit]);
        unit.parent = Some(Box::new(temp));
        temp.children.push(unit);
        return Node::new_tree(token, vec![unit]);
    }

    Node::new_leaf(token)
}

fn parse_expression() {}

fn parse(tokens: Vec<Token>) {
    let mut it = tokens.iter().peekable();
    //parse_number(it);
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::lexer::lexer;

    use super::*;

    #[test]
    fn parser_number_1() {
        let vec_num = lexer("42");
        let node = parse_number(vec_num.iter().peekable());
        assert_eq!(
            node,
            Node {
                token: vec_num[0],
                parent: None,
                children: Box::new(Vec::new()),
            }
        );
    }
    #[test]
    fn parser_number_2() {
        let vec_num = lexer("42K");
        let node = parse_number(vec_num.iter().peekable());
        assert_eq!(node.children[0].token.token_type, TokenType::Word("K"),);
        //assert_eq!(node.children[0].parent, Box::new(Some(node.clone())));
        // idk what happened here but ok
        assert_eq!(node.children[0].parent.as_ref().unwrap().deref(), &node);
    }

    #[test]
    fn parser_unit_1() {
        let vec_unit = lexer("K");
        let node = parse_unit(vec_unit.iter().peekable());
        assert_eq!(node.unwrap().token.token_type, TokenType::Word("K"));
    }

    #[test]
    fn parser_unit_2() {
        let vec_unit = lexer("");
        let node = parse_unit(vec_unit.iter().peekable());
        assert_eq!(node, None);
    }

    #[test]
    fn parser_unit_3() {
        let vec_unit = lexer("shoot");
        let node = parse_unit(vec_unit.iter().peekable());
        assert_eq!(node, None);
    }
}
