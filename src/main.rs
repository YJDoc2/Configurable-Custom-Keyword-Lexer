mod ast;
mod config;
mod eval;
mod lexer;
mod parser;
mod tokens;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(version = "0.0.1", author = "Yashodhan Joshi <yjdoc2@gmail.com>")]
struct Opts {
    /// Keyword Configuration file
    #[clap(short, long)]
    config: PathBuf,
    /// Source code file
    #[clap(short, long)]
    file: PathBuf,
}

fn main() {
    let opts = Opts::parse();
    let ip = std::fs::read_to_string(&opts.file).unwrap();
    let config = config::get_config(&opts.config).unwrap();
    let l = lexer::Lexer::new(ip.chars().collect(), config.clone());
    let ast = parser::programParser::new().parse(&config, l).unwrap();
    eval::eval(&ast);
}
