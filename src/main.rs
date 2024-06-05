use syntcalc as sc;
use std::io;
fn main() {
    let mut parser = sc::SyntCalc::new();
    parser.token_builder.insert_defaults();
    let mut string = String::new();
    loop {
        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read line");
        if string.trim() == "exit()" {break;}

        match parser.eval_str(&string) {
            Ok(val) => println!("{}", val),
            Err(e) => println!("{}", e),
        }
        string.clear();
    }
}
