use syntcalc as sc;
use std::{cell::RefCell, env::args, sync::Arc};
fn main() {
    let parser = sc::SyntCalc::new();
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
