use crate::config::LangConfig;
use crate::tokens::Token;
use deunicode::deunicode;
use std::collections::VecDeque;
use unicode_xid::UnicodeXID;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug)]
pub struct Lexer {
    ip: VecDeque<char>,
    config: LangConfig,
    count: usize,
}

#[derive(Debug)]
pub struct LexerError(String);

impl Lexer {
    pub fn new(ip: Vec<char>, config: LangConfig) -> Self {
        Self {
            ip: VecDeque::from(ip),
            config,
            count: 0,
        }
    }
}

impl Iterator for Lexer {
    type Item = Spanned<Token, usize, LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.ip.is_empty() {
            let c = self.ip.pop_front().unwrap();
            if c.is_whitespace() {
                self.count += 1;
                continue;
            }

            if c == '+' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::Plus, self.count)));
            }

            if c == '-' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::Minus, self.count)));
            }

            if c == '*' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::Mul, self.count)));
            }

            if c == '/' {
                if !self.ip.is_empty() && self.ip[0] == '/' {
                    while !self.ip.is_empty() && self.ip[0] != '\n' {
                        self.ip.pop_front();
                        self.count += 1;
                    }
                    self.count += 1; // for the /n
                    continue;
                }
                self.count += 1;
                return Some(Ok((self.count - 1, Token::Div, self.count)));
            }
            if c == '=' {
                self.count += 1;
                if !self.ip.is_empty() && self.ip[0] == '=' {
                    self.count += 1;
                    return Some(Ok((self.count - 2, Token::EqEq, self.count)));
                }
                return Some(Ok((self.count - 1, Token::Eq, self.count)));
            }

            if c == '>' {
                self.count += 1;
                if !self.ip.is_empty() && self.ip[0] == '=' {
                    self.count += 1;
                    return Some(Ok((self.count - 2, Token::GreaterThanEq, self.count)));
                }

                return Some(Ok((self.count - 1, Token::GreaterThan, self.count)));
            }

            if c == '<' {
                self.count += 1;
                if !self.ip.is_empty() && self.ip[0] == '=' {
                    self.count += 1;
                    return Some(Ok((self.count - 2, Token::LessThanEq, self.count)));
                }

                return Some(Ok((self.count - 1, Token::LessThan, self.count)));
            }
            if c == '!' {
                self.count += 1;
                if !self.ip.is_empty() && self.ip[0] == '=' {
                    self.count += 1;
                    return Some(Ok((self.count - 2, Token::NotEq, self.count)));
                }
                return Some(Ok((self.count - 1, Token::Not, self.count)));
            }

            if c == '"' {
                let mut t = Vec::new();
                let old = self.count;
                self.count += 1;
                while !self.ip.is_empty() && self.ip[0] != '"' {
                    self.count += 1;
                    t.push(self.ip.pop_front().unwrap());
                }
                // remove the ending "
                self.ip.pop_front();
                self.count += 1;
                return Some(Ok((
                    old,
                    Token::StringVal(t.into_iter().collect()),
                    self.count,
                )));
            }

            if c == '\'' {
                let mut t = Vec::new();
                let old = self.count;
                self.count += 1;
                while !self.ip.is_empty() && self.ip[0] != '\'' {
                    self.count += 1;
                    t.push(self.ip.pop_front().unwrap());
                }
                // remove the ending '
                self.ip.pop_front();
                self.count += 1;
                return Some(Ok((
                    old,
                    Token::StringVal(t.into_iter().collect()),
                    self.count,
                )));
            }

            if c == '(' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::OpenRoundBrack, self.count)));
            }

            if c == ')' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::CloseRoundBrack, self.count)));
            }

            if c == '{' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::OpenCurlyBrack, self.count)));
            }
            if c == '}' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::CloseCurlyBrack, self.count)));
            }

            if c == '[' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::OpenSquareBrack, self.count)));
            }
            if c == ']' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::CloseSquareBrack, self.count)));
            }

            if c == ';' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::SemiColon, self.count)));
            }

            if c == ',' {
                self.count += 1;
                return Some(Ok((self.count - 1, Token::Comma, self.count)));
            }

            if c.is_numeric() {
                let mut t = vec![c];
                let old = self.count;
                self.count += 1;
                while !self.ip.is_empty() && (self.ip[0].is_numeric() || self.ip[0] == '.') {
                    self.count += 1;
                    t.push(self.ip.pop_front().unwrap());
                }
                let _t: String = t.into_iter().collect();
                let _t = deunicode(&_t);

                return Some(Ok((old, Token::Number(_t), self.count)));
            }

            if UnicodeXID::is_xid_start(c) {
                let mut t = vec![c];
                let old = self.count;
                self.count += 1;

                while !self.ip.is_empty() && UnicodeXID::is_xid_continue(self.ip[0]) {
                    self.count += 1;
                    t.push(self.ip.pop_front().unwrap());
                }
                let _t: String = t.into_iter().collect();
                if self.config.is_keyword(&_t) {
                    let t = self.config.get_keyword(&_t).unwrap();
                    return Some(Ok((old, t, self.count)));
                }

                return Some(Ok((old, Token::ID(_t), self.count)));
            }
            let mut t = vec![c];
            while !self.ip.is_empty() && !self.ip[0].is_whitespace() {
                t.push(self.ip.pop_front().unwrap());
            }
            let _t: String = t.into_iter().collect();
            return Some(Err(LexerError(format!("Unknown Token `{}` found.", _t))));
        }
        None
    }
}
