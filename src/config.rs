use crate::tokens::Token;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs;

lazy_static! {
    static ref REQUIRED_TOKENS: HashMap<Token, &'static str> = {
        let mut t = HashMap::new();
        t.insert(Token::And, "&&");
        t.insert(Token::Or, "||");
        t.insert(Token::LetStart, "let");
        t.insert(Token::IfStart, "if");
        t.insert(Token::ElseStart, "else");
        t.insert(Token::PrintStart, "print");
        t.insert(Token::ForStart, "for");
        t.insert(Token::In, "in");
        t.insert(Token::WhileStart, "while");
        t
    };
}

#[derive(Debug, Clone)]
pub struct LangConfig {
    keywords: HashMap<String, Token>,
    empty_tokens: HashSet<Token>,
    set_tokens: HashSet<Token>,
}

impl Default for LangConfig {
    fn default() -> Self {
        LangConfig {
            keywords: REQUIRED_TOKENS
                .iter()
                .map(|(k, v)| ((*v).to_owned(), k.clone()))
                .collect(),
            empty_tokens: HashSet::new(),
            set_tokens: HashSet::new(),
        }
    }
}

pub fn get_config() -> Result<LangConfig, String> {
    let c = fs::read_to_string("config").unwrap();
    let mut lc = LangConfig {
        keywords: HashMap::new(),
        empty_tokens: HashSet::new(),
        set_tokens: HashSet::new(),
    };
    for kv in c.split('\n') {
        if kv.trim().is_empty() {
            continue;
        }
        let (k, v) = kv.split_once("=").unwrap();
        let token_type = Token::try_from(k.trim()).unwrap();
        if REQUIRED_TOKENS.contains_key(&token_type) && v.trim().is_empty() {
            return Err(format!(
                "Token `{}` is required, cannot be set to empty string",
                token_type
            ));
        }
        if v.trim().is_empty() {
            if lc.keywords.contains_key(v.trim()) && !REQUIRED_TOKENS.contains_key(&token_type) {
                return Err(format!(
                    "Cannot set same token for two keywords : `{}` used twice.",
                    v
                ));
            }
            lc.empty_tokens.insert(token_type.clone());
        } else {
            lc.keywords.insert(v.trim().to_owned(), token_type.clone());
            lc.set_tokens.insert(token_type);
        }
    }
    for (kw, default) in REQUIRED_TOKENS.iter() {
        if !lc.set_tokens.contains(kw) {
            lc.set_tokens.insert(kw.clone());
            lc.keywords.insert((*default).to_owned(), kw.clone());
        }
    }
    Ok(lc)
}

impl LangConfig {
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains_key(word)
    }

    pub fn get_keyword(&self, word: &str) -> Option<Token> {
        self.keywords.get(word).cloned()
    }

    pub fn is_optional(&self, t: Token) -> bool {
        !self.set_tokens.contains(&t)
    }
}
