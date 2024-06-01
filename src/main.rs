use pest::Parser;
use syntcalc as sc;
fn main() {
    let par = sc::MathParser::parse(sc::Rule::file, "3*fn(123.11E-2, (12+33)*2) + pi").unwrap();
    println!("{:#?}", par);
}
