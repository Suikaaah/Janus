#[allow(unused)]
mod ast_node;

use crate::tokenizer::{token::Token, Tokenizer};
use anyhow::{anyhow, bail, Result};
use ast_node::*;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Ast {
    source: LinkedList<Token>,
    tree: Option<Prog>,
}

impl Ast {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            source: tokenizer.into(),
            tree: None,
        }
    }

    fn advance(&mut self, len: usize) {
        for _ in 0..len {
            self.source.pop_front();
        }
    }

    fn next(&mut self) {
        self.advance(1);
    }

    fn step(&mut self, token: Token) -> Result<()> {
        match self.source.pop_front() {
            Some(x) if x == token => Ok(()),
            x => bail!("expected {token:?} found {x:?}"),
        }
    }

    fn step_identifier(&mut self) -> Result<String> {
        match self.source.pop_front() {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            x => bail!("expected an identifier found {x:?}"),
        }
    }

    fn front(&self) -> Result<&Token> {
        self.source
            .front()
            .ok_or_else(|| anyhow!("source is empty"))
    }

    pub fn build(&mut self) -> Result<()> {
        self.tree = Some(self.prog()?);
        Ok(())
    }

    fn prog(&mut self) -> Result<Prog> {
        Ok(Prog {
            p_main: self.p_main()?,
            ps: self.ps()?,
        })
    }

    fn p_main(&mut self) -> Result<Proc> {
        self.step(Token::Procedure)?;
        self.step(Token::Identifier("main".to_string()))?;
        self.step(Token::LParen)?;
        self.step(Token::RParen)?;

        let mut main_stuff = LinkedList::new();
        loop {
            match self.front()? {
                Token::Int => {
                    self.next();
                    main_stuff.push_back(MainStuff::Int(self.d()?));
                }
                Token::Stack => {
                    self.next();
                    main_stuff.push_back(MainStuff::Stack(self.x()?));
                }
                _ => break,
            }
        }

        Ok(Proc::Main {
            main_stuff,
            s: self.s()?,
        })
    }

    fn p(&mut self) -> Result<Proc> {
        self.step(Token::Procedure)?;
        let q = self.q()?;
        self.step(Token::LParen)?;

        let mut args = LinkedList::new();
        loop {
            match self.front()? {
                Token::Int | Token::Stack => {
                    args.push_back(Arg {
                        t: self.t()?,
                        x: self.x()?,
                    });
                    match self.front()? {
                        Token::Comma => self.next(),
                        Token::RParen => {}
                        x => bail!("expected comma or rparen found {x:?}"),
                    }
                }
                _ => {
                    self.step(Token::RParen)?;
                    break;
                }
            }
        }

        Ok(Proc::Other {
            q,
            args,
            s: self.s()?,
        })
    }

    fn ps(&mut self) -> Result<LinkedList<Proc>> {
        let mut value = LinkedList::new();
        loop {
            if self.source.is_empty() {
                break;
            } else {
                value.push_back(self.p()?);
            }
        }
        Ok(value)
    }

    fn t(&mut self) -> Result<Type> {
        let value = match self.front()? {
            Token::Int => Type::Int,
            Token::Stack => Type::Stack,
            x => bail!("expected type found {x:?}"),
        };
        self.next();
        Ok(value)
    }

    fn d(&mut self) -> Result<Vdec> {
        let x = self.x()?;
        if let Some(&Token::LSquareBracket) = self.source.front() {
            self.next();
            let c = self.c()?;
            self.step(Token::RSquareBracket)?;
            Ok(Vdec::Array { x, c })
        } else {
            Ok(Vdec::Scalar(x))
        }
    }

    fn x(&mut self) -> Result<Var> {
        Ok(Var(self.step_identifier()?))
    }

    fn q(&mut self) -> Result<PId> {
        Ok(PId(self.step_identifier()?))
    }

    fn s_non_recursive(&mut self) -> Result<Stm> {
        let call_or_uncall = {
            match self.front()? {
                Token::Call => |q, xs| Stm::Call { q, xs },
                Token::Uncall => |q, xs| Stm::Uncall { q, xs },
                _ => |_q, _xs| unreachable!(),
            }
        };

        match self.front()? {
            Token::Identifier(_) => {
                let x = self.x()?;
                if let Some(Token::LSquareBracket) = self.source.front() {
                    self.next();
                    let e_index = self.e()?;
                    self.step(Token::RSquareBracket)?;
                    let mod_op = self.mod_op()?;
                    let e = self.e()?;
                    Ok(Stm::AssignArray {
                        x,
                        e_index,
                        mod_op,
                        e,
                    })
                } else {
                    let mod_op = self.mod_op()?;
                    let e = self.e()?;
                    Ok(Stm::AssignScalar { x, mod_op, e })
                }
            }
            Token::If => {
                self.next();
                let e_if = self.e()?;
                self.step(Token::Then)?;
                let s_then = self.s()?;
                self.step(Token::Else)?;
                let s_else = self.s()?;
                self.step(Token::Fi)?;
                let e_fi = self.e()?;
                Ok(Stm::Conditional {
                    e_if,
                    s_then: Box::new(s_then),
                    s_else: Box::new(s_else),
                    e_fi,
                })
            }
            Token::From => {
                self.next();
                let e_from = self.e()?;
                self.step(Token::Do)?;
                let s_do = self.s()?;
                self.step(Token::Loop)?;
                let s_loop = self.s()?;
                self.step(Token::Until)?;
                let e_until = self.e()?;
                Ok(Stm::Loop {
                    e_from,
                    s_do: Box::new(s_do),
                    s_loop: Box::new(s_loop),
                    e_until,
                })
            }
            Token::Push => {
                self.next();
                self.step(Token::LParen)?;
                let left = self.x()?;
                self.step(Token::Comma)?;
                let right = self.x()?;
                self.step(Token::RParen)?;
                Ok(Stm::Push(left, right))
            }
            Token::Pop => {
                self.next();
                self.step(Token::LParen)?;
                let left = self.x()?;
                self.step(Token::Comma)?;
                let right = self.x()?;
                self.step(Token::RParen)?;
                Ok(Stm::Pop(left, right))
            }
            Token::Local => {
                self.next();
                let t_local = self.t()?;
                let x_local = self.x()?;
                self.step(Token::Equal)?;
                let e_local = self.e()?;
                let s = self.s()?;
                self.step(Token::Delocal)?;
                let t_delocal = self.t()?;
                let x_delocal = self.x()?;
                self.step(Token::Equal)?;
                let e_delocal = self.e()?;
                Ok(Stm::Local {
                    t_local,
                    x_local,
                    e_local,
                    s: Box::new(s),
                    t_delocal,
                    x_delocal,
                    e_delocal,
                })
            }
            Token::Call | Token::Uncall => {
                self.next();
                let q = self.q()?;
                self.step(Token::LParen)?;

                let mut xs = LinkedList::new();
                loop {
                    match self.front()? {
                        Token::RParen => {
                            self.next();
                            break;
                        }
                        _ => {
                            xs.push_back(self.x()?);
                            match self.front()? {
                                Token::Comma => self.next(),
                                Token::RParen => {}
                                x => bail!("expected comma or rparen found {x:?}"),
                            }
                        }
                    }
                }

                Ok(call_or_uncall(q, xs))
            }
            Token::Skip => {
                self.next();
                Ok(Stm::Skip)
            }
            x => bail!("expected non-recursive statement found {x:?}"),
        }
    }

    fn s(&mut self) -> Result<Stm> {
        let primary = self.s_non_recursive()?;
        let sequence = self
            .source
            .front()
            .map(|token| {
                matches!(
                    token,
                    Token::Identifier(_)
                        | Token::If
                        | Token::From
                        | Token::Push
                        | Token::Local
                        | Token::Call
                        | Token::Uncall
                        | Token::Skip
                )
            })
            .unwrap_or(false);
        if sequence {
            Ok(Stm::Sequence(Box::new(primary), Box::new(self.s()?)))
        } else {
            Ok(primary)
        }
    }

    fn c(&mut self) -> Result<Con> {
        match self.front()? {
            Token::Constant(c) => {
                let con = Con(*c);
                self.next();
                Ok(con)
            }
            x => bail!("expected constant found {x:?}"),
        }
    }

    fn e_non_recursive(&mut self) -> Result<Exp> {
        match self.front()? {
            Token::Constant(_) => Ok(Exp::Constant(self.c()?)),
            Token::Identifier(_) => {
                let x = self.x()?;
                if let Some(Token::LSquareBracket) = self.source.front() {
                    self.next();
                    let e = self.e()?;
                    self.step(Token::RSquareBracket)?;
                    Ok(Exp::Indexed { x, e: Box::new(e) })
                } else {
                    Ok(Exp::Variable(x))
                }
            }
            Token::Empty => {
                self.next();
                self.step(Token::LParen)?;
                let x = self.x()?;
                self.step(Token::RParen)?;
                Ok(Exp::Empty(x))
            }
            Token::Top => {
                self.next();
                self.step(Token::LParen)?;
                let x = self.x()?;
                self.step(Token::RParen)?;
                Ok(Exp::Top(x))
            }
            Token::Nil => {
                self.next();
                Ok(Exp::Nil)
            }
            x => bail!("expected non-recursive expression found {x:?}"),
        }
    }

    fn e(&mut self) -> Result<Exp> {
        let primary = self.e_non_recursive()?;
        let bin_op = self
            .source
            .front()
            .map(|token| {
                matches!(
                    token,
                    Token::Plus
                        | Token::Minus
                        | Token::Caret
                        | Token::Asterisk
                        | Token::Slash
                        | Token::Percent
                        | Token::Ampersand
                        | Token::VerticalBar
                        | Token::Ampersand2
                        | Token::VerticalBar2
                        | Token::Less
                        | Token::Greater
                        | Token::Equal
                        | Token::ExclamationEqual
                        | Token::LessEqual
                        | Token::GreaterEqual
                )
            })
            .unwrap_or(false);
        if bin_op {
            Ok(Exp::BinOp(
                Box::new(primary),
                self.op()?,
                Box::new(self.e()?),
            ))
        } else {
            Ok(primary)
        }
    }

    fn mod_op(&mut self) -> Result<ModOp> {
        let value = match self.front()? {
            Token::PlusEqual => ModOp::Add,
            Token::MinusEqual => ModOp::Sub,
            Token::CaretEqual => ModOp::Xor,
            x => bail!("expected mod_op found {x:?}"),
        };

        self.next();
        Ok(value)
    }

    fn op(&mut self) -> Result<Op> {
        let value = match self.front()? {
            Token::Plus => Op::Add,
            Token::Minus => Op::Sub,
            Token::Caret => Op::Xor,
            Token::Asterisk => Op::Mul,
            Token::Slash => Op::Div,
            Token::Percent => Op::Mod,
            Token::Ampersand => Op::And,
            Token::VerticalBar => Op::Or,
            Token::Ampersand2 => Op::And2,
            Token::VerticalBar2 => Op::Or2,
            Token::Less => Op::Less,
            Token::Greater => Op::Greater,
            Token::Equal => Op::Equal,
            Token::ExclamationEqual => Op::NotEqual,
            Token::LessEqual => Op::LessEqual,
            Token::GreaterEqual => Op::GreaterEqual,
            x => bail!("expected op found {x:?}"),
        };

        self.next();
        Ok(value)
    }
}
