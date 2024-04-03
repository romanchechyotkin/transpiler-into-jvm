use compiler_in_rust::parser::parser::Parser;

fn main() {
    let input = "var wasd = 5; var a = wasd;";

    let mut parser = Parser::new(input.to_string());
    let program = parser.parse_program();

    for stmt in &program {
        dbg!(stmt);
    }
}
