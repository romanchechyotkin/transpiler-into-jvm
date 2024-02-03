use compiler_in_rust::lexer::lexer::Lexer;
use compiler_in_rust::tokens::tokens::Token;

fn main() {
    let input = "+-? = / *++ * != asdddhghj  == = ! 1 123213 1 123 yu asd 
        faxx fax return reutnt fn func cap if else 
    ";

    let mut lex = Lexer::new(input.to_string());
    while let Ok(tok) = lex.next_token() {
        println!("{}", tok);
        if let Token::Eof = tok {
            break;
        }
    }
}
