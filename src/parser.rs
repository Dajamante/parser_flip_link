use std::ops::Range;

use crate::lexer::{self, lexer};
use lexer::{Token, TokenKind::*};
type Span = Range<usize>;
use anyhow::{anyhow, bail, Context, Error, Result};

const script: &str = "MEMORY
{
RAM (rx) : ORIGIN = 0x20000000 + 128K, LENGTH = 65536 + 128K  
FLASH : ORIGIN = 0x00000000, LENGTH = 262144
  
}
";

struct Expr {
    span: Span,
    kind: ExprKind,
}

enum ExprKind {
    Symbol(String),
    Number(u64),        // 128
    Binary(BinaryExpr), // 1 + 1
    Paren(ParenExpr),   // (1 + 2)
}

struct BinaryExpr {
    lhs: Box<Expr>,
    op: Operator,
    rhs: Box<Expr>,
}

struct ParenExpr {
    inner: Box<Expr>,
}

enum Operator {
    Plus,
    Minus,
}

pub fn parse(tokens: Vec<Token>) -> Result<()> {
    let mut stack_commands: Vec<Expr> = Vec::new();
    let tokens = lexer(script);

    let mut it = tokens.into_iter();
    while let Some(token) = it.next() {
        let mut stack: Vec<Token> = Vec::new();

        match token.token_kind {
            Number(num) => {
                let num = ExprKind::try_from(lexer::TokenKind::Number(num))?;
                let operand1 = Expr {
                    span: token.span,
                    kind: num,
                };
                if let Some(next) = it.next() {
                    match next.token_kind {
                        Plus => {
                            if let Some(num) = it.next() {
                                let num = ExprKind::try_from(num.token_kind)?;
                                let operand2 = Expr {
                                    span: token.span,
                                    kind: num,
                                };
                                stack_commands.push(Expr {
                                    span: operand1.span.start..operand2.span.end,
                                    kind: ExprKind::Binary(BinaryExpr {
                                        lhs: Box::new(operand1),
                                        op: Operator::try_from(Plus)?,
                                        rhs: Box::new(operand2),
                                    }),
                                });
                            } // binary expression -> BinaryExpr
                        }
                        _ => todo!(),
                    }
                }
            }
            Plus => {
                if let Some(exp) = stack_commands.pop() {
                    match exp.kind {
                        ExprKind::Binary(_) | ExprKind::Number(_) => {
                            if let Some(operand) = it.next() {}
                        }

                        _ => bail!("Binary or number must precede a plus"),
                    }
                }
            }

            Colon => todo!(),
            CurlyClose => todo!(),
            CurlyOpen => todo!(),
            Equal => todo!(),
            Word(_) => todo!(),
            Comma => todo!(),
            Dot => todo!(),
            ParClose => todo!(),
            ParOpen => todo!(),
        }
    }
    Ok(())
}

impl TryFrom<lexer::TokenKind> for ExprKind {
    type Error = anyhow::Error;
    fn try_from(token_kind: lexer::TokenKind) -> Result<Self> {
        match token_kind {
            lexer::TokenKind::Number(n) => Ok(ExprKind::Number(n)),
            _ => bail!("That's not a number"),
        }
    }
}

impl TryFrom<lexer::TokenKind> for Operator {
    type Error = anyhow::Error;
    fn try_from(token_kind: lexer::TokenKind) -> Result<Self> {
        match token_kind {
            lexer::TokenKind::Plus => Ok(Operator::Plus),
            _ => bail!("That's not an operator"),
        }
    }
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
