use compiler_in_rust::parser::parser::Parser;
use compiler_in_rust::generator::generator::{Generator, JVMInstruction};
use std::process::{Command, Output};

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

    let mut instructions: Vec<JVMInstruction> = Vec::new(); 

    for stmt in &program {
        let mut ins = generator.generate_code(stmt);
        instructions.append(&mut ins);
    }
    
    dbg!(&instructions);

    cmd()
}

fn cmd() {
    let mut output = Command::new("javac")
        .arg("-cp")
        .arg("bcel-6.8.2.jar")
        .arg("BCELMainGenerator.java")
        .output()
        .expect("failed to run javac");

    print_output(&output);


    output = Command::new("java")
        .arg("-cp")
        .arg(".:bcel-6.8.2.jar")
        .arg("BCELMainGenerator")
        .output()
        .expect("failed to run java");

    print_output(&output);

    output = Command::new("java")
        .arg("Main")
        .output()
        .expect("failed to run java");

    print_output(&output);
}

fn print_output(output: &Output) {
    if output.status.success() {
        println!("Command executed successfully!");
        println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Command failed with status: {}", output.status);
        println!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}