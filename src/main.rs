mod ast;
mod config;
mod eval;
mod lexer;
mod parser;
mod tokens;
use std::collections::HashMap;
fn main() {
    let ip = "नवीन v1 = १२३४; हे(v1+(123 - (25*6)))दाखवा";
    let config = config::get_config().unwrap();
    // let mut stab = HashMap::new();
    let l = lexer::Lexer::new(ip.chars().collect(), config.clone());
    let ast = parser::programParser::new().parse(&config, l).unwrap();
    eval::eval(ast);
}
