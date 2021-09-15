mod ast;
mod config;
mod eval;
mod lexer;
mod parser;
mod tokens;
fn main() {
    let ip =
        "नवीन v1 = १२३४; जर (v1>12) तर{ हे(v1+(123 - (25*6))) दाखवा } नाहीतर { हे('Hello') दाखवा } v1 = १; जर (v1>12) तर{ हे(v1+(123 - (25*6))) दाखवा } नाहीतर { हे('Hello') दाखवा } ";
    let config = config::get_config().unwrap();
    let l = lexer::Lexer::new(ip.chars().collect(), config.clone());
    let ast = parser::programParser::new().parse(&config, l).unwrap();
    eval::eval(ast);
}
