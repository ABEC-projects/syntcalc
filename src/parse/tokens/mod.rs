pub mod val;

use regex::Regex;
pub use val::Val as Val;
pub enum Tokens{
    Op(Operator),
    Val(Val),
    Brc(String),
    Fn(Function),
}

pub fn tokenize(expr: &str) -> Result<Vec<Tokens>, String>{
    let pv = Vec::new();
    let regex_val = Regex::new(r"^(?<val>\d+(\.\d+(E-?\d+)?)?)").unwrap();
    let Some(caps) = regex_val.captures(expr) else {todo!()};
    
    return Ok(pv);
}

pub struct Operator {
    op: Ops,
}
enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}
impl Operator {
    pub fn new(op: Ops) -> Self{
        Operator { op }
    }
    pub fn from_str(s: &str) -> Result<Self, String>{
        use Ops::*;
        match match s{
            "+" => Some(Add),
            "-" => Some(Sub),
            "*" => Some(Mul),
            "/" => Some(Div),
            "^" => Some(Pow),
            "**" => Some(Pow),
            "%" => Some(Mod),
            _ => None,
        }{
        Some(op) => Ok(Self::new(op)),
        None => Err(format!("No operator for {}", s)),
        }
    }
}



pub struct Function{
    lambda: fn(Vec<Val>) -> Val,
    arg_number: i32,
}
impl Function{
    pub fn new(lambda: fn(Vec<Val>) -> Val,  arg_number: i32) -> Self{
        Function{lambda, arg_number}
    }
    pub fn compute (&self,args: Vec<Val>) -> Result<Val, &'static str> {
        if args.len() ==  self.arg_number as usize{
            return Ok((self.lambda)(args));
        }else{
            return Err("Argument number do not match");
        }
    }
}
