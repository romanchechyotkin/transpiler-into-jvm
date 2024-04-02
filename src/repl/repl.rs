use compiler_in_rust::parser::parser::Parser;

fn main() {
    let input = "var = +-? = / *++ * != asdddhghj  == = ;;; ! 1 123213 1 123 yu asd 
        faxx fax return reutnt fn func cap if else var VAR 
    ";

    let mut parser = Parser::new(input.to_string());
    parser.parse_program();
}
