use crate::tokens::Token;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs;

lazy_static! {
    static ref REQUIRED_TOKENS: Vec<Token> = vec![
        Token::And,
        Token::Or,
        Token::PrintStart,
        Token::ForStart,
        Token::In,
        Token::IfStart,
        Token::LetStart,
        Token::WhileStart,
    ];
}

#[derive(Debug)]
pub struct LangConfig {
    keywords: HashMap<String, Token>,
    empty_tokens: HashSet<Token>,
}

impl Default for LangConfig {
    fn default() -> Self {
        let mut t = HashMap::new();
        t.insert("&&".to_owned(), Token::And);
        t.insert("||".to_owned(), Token::Or);
        t.insert("let".to_owned(), Token::LetStart);
        t.insert("if".to_owned(), Token::IfStart);
        t.insert("print".to_owned(), Token::PrintStart);
        t.insert("for".to_owned(), Token::ForStart);
        t.insert("in".to_owned(), Token::In);
        t.insert("While".to_owned(), Token::WhileStart);
        LangConfig {
            keywords: t,
            empty_tokens: HashSet::new(),
        }
    }
}

pub fn get_config() -> Result<LangConfig, String> {
    let c = fs::read_to_string("config").unwrap();
    let mut lc = LangConfig::default();
    for kv in c.split('\n') {
        if kv.trim().len() == 0 {
            continue;
        }
        let (k, v) = kv.split_once("=").unwrap();
        let token_type = Token::try_from(k.trim()).unwrap();
        if REQUIRED_TOKENS.contains(&token_type) && v.trim().len() == 0 {
            return Err(format!(
                "Token `{}` is required, cannot be set to empty string",
                token_type
            ));
        }
        if v.trim().len() == 0 {
            lc.empty_tokens.insert(token_type.clone());
        }
        lc.keywords.insert(v.trim().to_owned(), token_type);
    }
    Ok(lc)
}

impl LangConfig {
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains_key(word)
    }

    pub fn get_keyword(&self, word: &str) -> Option<Token> {
        match self.keywords.get(word) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }
}
