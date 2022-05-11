use std::ops::Range;

use crate::lexer::{self, lexer, Span, TokenKind};
use anyhow::{anyhow, bail, Context, Error, Result};
use lexer::{Token, TokenKind::*};

// const script: &str = "MEMORY
// {
// RAM (rx) : ORIGIN = 0x20000000 + 128K, LENGTH = 65536 + 128K
// FLASH : ORIGIN = 0x00000000, LENGTH = 262144

// }
// ";

struct Node {
    token: Token,
    children: Vec<Box<Node>>,
}

// int parse(int start, InfixParseResult& result, const OpMap& availableOps) {
// 	//Program stack;
// 	//const auto& tokens = result.tokens;
// 	//size_t i = start;
// 	for (; i < tokens.size(); i++) {
// 		const auto& token = tokens[i];
// 		if (token.isOp()) {
// 			const Op* op = getOp(availableOps, token.token);
// 			if (op != nullptr) {
// 				if (op->getStackRequire() == 0) {
// 					result.program.push(op);
// 				}
// 				else {
// 					if (!stack.isEmpty() && getPrecedence(stack.top()) > getPrecedence(op)) {
// 						while (!stack.isEmpty()) {
// 							result.program.push(stack.top());
// 							stack.pop();
// 						}
// 					}
// 					stack.push(op);
// 				}
// 			}
// 			else {
// 				result.errors.push_back("Unknown identifier '" + token.token + "'.");
// 			}
// 		}

// 		if (token.token == "(" || token.token == "[") {
// 			i = parse(i + 1, result, availableOps);
// 		}
// 		else if (token.token == ")" || token.token == "]") {
// 			break;
// 		}
// 	}

// 	while (!stack.isEmpty()) {
// 		result.program.push(stack.top());
// 		stack.pop();
// 	}
// 	return i;
// }

fn get_stack_requirement(token: &Token) -> usize {
    match &token.token_kind {
        Plus => 2,
        Equal => 2,
        Word(w) if *w == "RAM".to_string() => 2,
        _ => 0,
    }
}

fn is_relevant(t: &Token) -> bool {
    (t.token_kind != TokenKind::ParClose) && (t.token_kind != TokenKind::ParOpen)
}

fn parse_sub(start: usize, tokens: &Vec<Token>, postfix: &mut Vec<Token>) -> usize {
    println!("{:#?}", tokens);
    let mut index = start;
    let mut stack: Vec<Token> = Vec::new();
    while index < tokens.len() {
        let t = &tokens[index];
        // om jag skippar parentes här
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

pub fn parse(script: &str) -> Vec<Token> {
    let tokens = lexer(script);
    let mut postfix: Vec<Token> = Vec::new();
    println!("POSTFIX   !!{:#?}", postfix);

    parse_sub(0, &tokens, &mut postfix);

    postfix
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
