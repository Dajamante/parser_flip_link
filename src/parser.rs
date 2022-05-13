use std::{collections::VecDeque, ops::Range};

use crate::lexer::{self, lexer, Span, TokenKind};
use anyhow::{anyhow, bail, Context, Error, Result};
use lexer::{Token, TokenKind::*};

// const script: &str = "MEMORY
// {
// RAM (rx) : ORIGIN = 0x20000000 + 128K, LENGTH = 65536 + 128K
// FLASH : ORIGIN = 0x00000000, LENGTH = 262144

// }
// ";

#[derive(Debug)]
struct Node {
    token: Token,
    children: Vec<Box<Node>>,
}
/// Tokens have different requirements, for example:
/// - "+" has requirement 2 numbers
/// - A number has zero requirements
fn get_stack_requirement(token: &Token) -> usize {
    match &token.token_kind {
        Plus => 2,
        Equal => 2,
        Word(w) if *w == "RAM".to_string() => 2,
        _ => 0,
    }
}

/// This method skips parenthesis and everything not relevant for the tree
fn is_relevant(t: &Token) -> bool {
    (t.token_kind != TokenKind::ParClose) && (t.token_kind != TokenKind::ParOpen)
}

/// This method recursively creates a postfix vector of Tokens
fn parse_sub(start: usize, tokens: &Vec<Token>, postfix: &mut Vec<Token>) -> usize {
    println!("{:#?}", tokens);
    let mut index = start;
    let mut stack: Vec<Token> = Vec::new();
    while index < tokens.len() {
        let t = &tokens[index];
        if is_relevant(t) {
            if get_stack_requirement(t) == 0 {
                postfix.push(t.clone());
            } else {
                while !stack.is_empty() {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(t.clone());
            }
        }
        if t.token_kind == ParOpen {
            index = parse_sub(index + 1, tokens, postfix);
        } else if t.token_kind == ParClose {
            break;
        }
        index += 1;
    }

    while !stack.is_empty() {
        postfix.push(stack.pop().unwrap());
    }
    return index;
}

/// This method puts the tokens in the right order to build a tree and returns a Vec<Token>
pub fn parse(script: &str) -> Vec<Token> {
    let tokens = lexer(script);
    let mut postfix: Vec<Token> = Vec::new();

    parse_sub(0, &tokens, &mut postfix);

    postfix
}

fn build_tree(postfix: Vec<Token>) -> Vec<Node> {
    let mut stack: Vec<Node> = Vec::new();
    for t in postfix {
        let mut n = Node {
            token: t.clone(),
            children: Vec::new(),
        };
        for _ in 0..get_stack_requirement(&t) {
            let temp = stack.pop().unwrap();
            n.children.insert(0, Box::new(temp));
        }
        stack.push(n);
    }
    println!("{:#?}", &stack);
    Vec::from_iter(stack)
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer;

    use super::*;

    #[test]
    fn parser_postfix_1() {
        assert!(is_equal(parse("1 + 2"), vec![Number(1), Number(2), Plus]));
    }

    #[test]
    fn parser_postfix_2() {
        assert!(is_equal(
            parse("1 + 2 + 3"),
            vec![Number(1), Number(2), Plus, Number(3), Plus]
        ));
    }

    #[test]
    fn parser_postfix_3() {
        assert!(!is_equal(
            parse("1 + 2 + 3"),
            vec![Number(1), Number(5), Plus, Number(3), Plus]
        ));
    }

    #[test]
    fn parser_postfix_4_par() {
        assert!(is_equal(parse("(1)"), vec![Number(1)]));
    }

    #[test]
    fn parser_postfix_5() {
        assert!(is_equal(
            parse("1 + (2 + 3)"),
            vec![Number(1), Number(2), Number(3), Plus, Plus]
        ));
    }
    /*
    "1 + 2)"
    Becomes in postfix -> 1 2 +
    Becomes a tree:
            +
           / \
          1   2
    */
    #[test]
    fn build_tree_1() {
        let postfix = parse("1 + 2");
        let tree = build_tree(postfix);
        assert!(tree[0].token.token_kind == TokenKind::Plus);
        assert!(tree[0].children[0].token.token_kind == TokenKind::Number(1));
        assert!(tree[0].children[1].token.token_kind == TokenKind::Number(2));
    }

    /*
    "1 + (3 + 4)"
    Becomes in postfix -> 1 3 4 + +
    Becomes a tree:
            +
           / \
          1   +
             / \
            3   4
    */
    #[test]
    fn build_tree_2() {
        let postfix = parse("1 + (3 + 4)");
        let tree = build_tree(postfix);
        assert!(tree[0].token.token_kind == TokenKind::Plus);
        assert!(tree[0].children[0].token.token_kind == TokenKind::Number(1));
        assert!(tree[0].children[1].token.token_kind == TokenKind::Plus);
        assert!(tree[0].children[1].children[0].token.token_kind == TokenKind::Number(3));
        assert!(tree[0].children[1].children[1].token.token_kind == TokenKind::Number(4));
    }

    // Two roots
    #[test]
    fn build_tree_3() {
        let postfix = parse("1 2");
        let tree = build_tree(postfix);
        assert!(tree[0].token.token_kind == TokenKind::Number(1));
        assert!(tree[1].token.token_kind == TokenKind::Number(2));
    }
}

fn is_equal(v1: Vec<Token>, v2: Vec<TokenKind>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i].token_kind != v2[i] {
            return false;
        }
    }
    return true;
}
