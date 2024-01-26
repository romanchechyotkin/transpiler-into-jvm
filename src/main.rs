use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    loop {
        let mut buffer = String::new();
        handle.read_line(&mut buffer)?;   
        println!("{}", buffer);
    }
}
