use compiler_in_rust::parser::parser::Parser;

fn main() {
    let input = "+-? = / *++ * != asdddhghj  == = ! 1 123213 1 123 yu asd 
        faxx fax return reutnt fn func cap if else 
    ";

    let mut parser = Parser::new(input.to_string());
    parser.parse_program();
}
