use compiler_in_rust::parser::parser::Parser;
use compiler_in_rust::generator::generator::Generator;

fn main() {
    let input = "
    var wasd = 5; 
    var a = wasd; 
    var aa = a;   
    var aaa = wasd; 
";

    let mut parser = Parser::new(input.to_string());
    let program = parser.parse_program();

    let mut generator: Generator = Generator::new();
    
    for stmt in &program {
        let instructions = generator.generate_code(stmt);
        dbg!(instructions);
    }

}
