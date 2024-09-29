pub mod token;

use crate::util::char_list;
use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, LinkedList};
use token::Token;

#[derive(Debug)]
pub struct Tokenizer {
    source: LinkedList<char>,
    tokens: LinkedList<Token>,
    buffer: LinkedList<char>,
}

impl Tokenizer {
    pub fn new(source: LinkedList<char>) -> Self {
        Self {
            source,
            tokens: Default::default(),
            buffer: Default::default(),
        }
    }

    fn take_buffer(&mut self) -> String {
        std::mem::take(&mut self.buffer).into_iter().collect()
    }

    fn push_buffer(&mut self) -> Result<()> {
        lazy_static! {
            static ref keywords: BTreeMap<LinkedList<char>, Token> = {
                let mut value = BTreeMap::new();
                value.insert(char_list("int"), Token::Int);
                value.insert(char_list("stack"), Token::Stack);
                value.insert(char_list("procedure"), Token::Procedure);
                value.insert(char_list("if"), Token::If);
                value.insert(char_list("then"), Token::Then);
                value.insert(char_list("else"), Token::Else);
                value.insert(char_list("fi"), Token::Fi);
                value.insert(char_list("from"), Token::From);
                value.insert(char_list("do"), Token::Do);
                value.insert(char_list("loop"), Token::Loop);
                value.insert(char_list("until"), Token::Until);
                value.insert(char_list("push"), Token::Push);
                value.insert(char_list("pop"), Token::Pop);
                value.insert(char_list("local"), Token::Local);
                value.insert(char_list("delocal"), Token::Delocal);
                value.insert(char_list("call"), Token::Call);
                value.insert(char_list("uncall"), Token::Uncall);
                value.insert(char_list("skip"), Token::Skip);
                value.insert(char_list("empty"), Token::Empty);
                value.insert(char_list("top"), Token::Top);
                value.insert(char_list("nil"), Token::Nil);
                value
            };
        }

        if let Some(character) = self.buffer.front() {
            let is_literal = ['\"', '\'', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
                .iter()
                .any(|c| c == character);

            let token = match (is_literal, keywords.get(&self.buffer)) {
                (true, _) => Token::Constant(self.take_buffer().parse()?),
                (_, None) => Token::Identifier(self.take_buffer()),
                (_, Some(token)) => {
                    self.buffer.clear();
                    token.clone()
                }
            };
            self.push_token(token);
        }

        Ok(())
    }

    fn push_token(&mut self, token: Token) {
        self.tokens.push_back(token);
    }

    fn advance(&mut self, len: usize) {
        for _ in 0..len {
            self.source.pop_front();
        }
    }

    fn front(&self) -> Option<char> {
        self.source.front().copied()
    }

    fn get(&self, index: usize) -> Option<char> {
        self.source.iter().nth(index).copied()
    }

    pub fn tokenize(&mut self) -> Result<()> {
        lazy_static! {
            static ref table_1: BTreeMap<char, Token> = {
                let mut value = BTreeMap::new();
                value.insert('+', Token::Plus);
                value.insert('-', Token::Minus);
                value.insert('^', Token::Caret);
                value.insert('*', Token::Asterisk);
                value.insert('/', Token::Slash);
                value.insert('%', Token::Percent);
                value.insert('&', Token::Ampersand);
                value.insert('|', Token::VerticalBar);
                value.insert('<', Token::Less);
                value.insert('>', Token::Greater);
                value.insert('=', Token::Equal);
                value.insert('(', Token::LParen);
                value.insert(')', Token::RParen);
                value.insert('[', Token::LSquareBracket);
                value.insert(']', Token::RSquareBracket);
                value.insert(',', Token::Comma);
                value
            };
            static ref table_2: BTreeMap<(char, char), Token> = {
                let mut value = BTreeMap::new();
                value.insert(('&', '&'), Token::Ampersand2);
                value.insert(('|', '|'), Token::VerticalBar2);
                value.insert(('!', '='), Token::ExclamationEqual);
                value.insert(('<', '='), Token::LessEqual);
                value.insert(('>', '='), Token::GreaterEqual);
                value.insert(('+', '='), Token::PlusEqual);
                value.insert(('-', '='), Token::MinusEqual);
                value.insert(('^', '='), Token::CaretEqual);
                value
            };
        }

        let front = match self.front() {
            None => {
                self.push_buffer()?;
                return Ok(());
            }
            Some(character) => character,
        };

        if [' ', '\n', '\t', '\r'].iter().any(|&c| c == front) {
            self.push_buffer()?;
            self.advance(1);
            self.tokenize()?;
            return Ok(());
        }

        if let Some(second) = self.get(1) {
            if let Some(token) = table_2.get(&(front, second)) {
                self.push_buffer()?;
                self.push_token(token.clone());
                self.advance(2);
                self.tokenize()?;
                return Ok(());
            }
        }

        if let Some(token) = table_1.get(&front) {
            self.push_buffer()?;
            self.push_token(token.clone());
            self.advance(1);
            self.tokenize()?;
            return Ok(());
        }

        self.buffer.push_back(front);
        self.advance(1);
        self.tokenize()?;
        Ok(())
    }
}

impl From<Tokenizer> for LinkedList<Token> {
    fn from(value: Tokenizer) -> Self {
        value.tokens
    }
}
