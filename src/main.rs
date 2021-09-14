mod config;
mod lexer;
mod tokens;
fn main() {
    let ip = "नवीन _abc = 5123; नवीन efg = १२३४; // this should be ignored \n जर (5>4 किंवा 3<2) तर {हे Hello दाखवा}नाहीतर{हे ('? , öäüßÜÖÄẞ') दाखवा}";
    let config = config::get_config().unwrap();
    let mut l = lexer::Lexer::new(ip.chars().collect(), config);
    let mut t = l.next();
    while t != tokens::Token::EOI {
        println!("token: {}", t);
        t = l.next();
    }
}
