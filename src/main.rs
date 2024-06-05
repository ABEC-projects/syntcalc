use pest::Parser;
use syntcalc as sc;
use std::io;
use std::env::args;
fn main() {
    let opts = sc::parse::ValOpts::default();
    let parser = sc::SyntCalc::new(&opts);
    let mut string = String::new();
    for i in args().skip(1) {
        string = format!("{} {}", string, i);
    }
    let result = parser.eval_str(&string);
    match result{
        Ok(v) => println!("{}", v.get_magnetude()),
        Err(e) => println!("{}", e),
    }
}
