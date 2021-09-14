use crate::config::LangConfig;
use crate::tokens::Token;
use deunicode::deunicode;
use std::collections::VecDeque;
use unicode_xid::UnicodeXID;

#[derive(Debug)]
pub struct Lexer {
    ip: VecDeque<char>,
    config: LangConfig,
    queue: Vec<Token>,
}

impl Lexer {
    pub fn new(ip: Vec<char>, config: LangConfig) -> Self {
        Self {
            ip: VecDeque::from(ip),
            config,
            queue: Vec::new(),
        }
    }
    pub fn next(&mut self) -> Token {
        while self.ip.len() != 0 {
            let c = self.ip.pop_front().unwrap();
            if c.is_whitespace() {
                continue;
            }

            if c == '+' {
                return Token::Plus;
            }

            if c == '-' {
                return Token::Minus;
            }

            if c == '*' {
                return Token::Mul;
            }

            if c == '/' {
                if self.ip.len() > 0 && self.ip[0] == '/' {
                    while self.ip.len() > 0 && self.ip[0] != '\n' {
                        self.ip.pop_front();
                    }
                    continue;
                }
                return Token::Div;
            }
            if c == '=' {
                if self.ip.len() > 0 && self.ip[0] == '=' {
                    return Token::EqEq;
                }
                return Token::Eq;
            }

            if c == '>' {
                if self.ip.len() > 0 && self.ip[0] == '=' {
                    return Token::GreaterThanEq;
                }
                return Token::GreaterThan;
            }

            if c == '<' {
                if self.ip.len() > 0 && self.ip[0] == '=' {
                    return Token::LessThanEq;
                }
                return Token::LessThan;
            }
            if c == '!' {
                if self.ip.len() > 0 && self.ip[0] == '=' {
                    return Token::NotEq;
                }
                return Token::Not;
            }

            if c == '"' {
                let mut t = Vec::new();
                while self.ip.len() > 0 && self.ip[0] != '"' {
                    t.push(self.ip.pop_front().unwrap());
                }
                // remove the ending "
                self.ip.pop_front();
                return Token::StringVal(t.into_iter().collect());
            }

            if c == '\'' {
                let mut t = Vec::new();
                while self.ip.len() > 0 && self.ip[0] != '\'' {
                    t.push(self.ip.pop_front().unwrap());
                }
                // remove the ending "
                self.ip.pop_front();
                return Token::StringVal(t.into_iter().collect());
            }

            if c == '(' {
                return Token::OpenRoundBrack;
            }

            if c == ')' {
                return Token::CloseRoundBrack;
            }

            if c == '{' {
                return Token::OpenCurlyBrack;
            }
            if c == '}' {
                return Token::CloseCurlyBrack;
            }

            if c == ';' {
                return Token::SemiColon;
            }

            if c == ',' {
                return Token::Comma;
            }

            if c.is_numeric() {
                let mut t = Vec::new();
                t.push(c);
                while self.ip.len() > 0 && (self.ip[0].is_numeric() || self.ip[0] == '.') {
                    t.push(self.ip.pop_front().unwrap());
                }
                let _t: String = t.into_iter().collect();
                let _t = deunicode(&_t);
                return Token::Number(_t.parse().unwrap());
            }
            if UnicodeXID::is_xid_start(c) {
                let mut t = Vec::new();
                t.push(c);
                while self.ip.len() > 0 && UnicodeXID::is_xid_continue(self.ip[0]) {
                    t.push(self.ip.pop_front().unwrap());
                }
                let _t: String = t.into_iter().collect();
                if self.config.is_keyword(&_t) {
                    return self.config.get_keyword(&_t).unwrap();
                }
                return Token::ID(_t);
            }
            return Token::Unknown;
        }
        Token::EOI
    }
}
