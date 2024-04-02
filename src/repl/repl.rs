use compiler_in_rust::parser::parser::Parser;

fn main() {
    let input = "var wasd = 5;";

    let mut parser = Parser::new(input.to_string());
    let program = parser.parse_program();

    dbg!(&program[0]);
}
