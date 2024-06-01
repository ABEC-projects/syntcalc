pub mod val;
pub mod token_builder;
pub mod associations;

use val::ValComputeError;
use regex::Regex;
pub use val::Val as Val;
pub enum Tokens{
    Op(Operator),
    Val(Val),
    Brc(Brace),
    Fn(Function),
}


/// A binary operator struct
pub struct Operator {
    kind: Ops,
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
    fn new(op: Ops) -> Self{
        Operator { kind: op }
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
            None => Err(format!("No operator for {s}")),
        }
    }
    pub fn compute(&self, lhs: Val, rhs: Val) -> Result<Val, ValComputeError> {
        use Ops::*;
        match self.kind {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => Ok(lhs * rhs),
            Div => Ok(lhs / rhs),
            Pow => lhs.pow_val(&rhs),
            Mod => todo!(),
        }
    }
}


#[derive(Clone, Copy)]
pub struct Function{
    lambda: fn(Vec<Val>) -> Val,
    argc: i32,
}
use self::associations::FnAlias;
impl Function{
    pub fn new(lambda: fn(Vec<Val>) -> Val,  argc: i32) -> Self{
        Function{lambda, argc}
    }
    pub fn compute (&self,args: Vec<Val>) -> Result<Val, &'static str> {
        if args.len() ==  self.argc as usize{
            return Ok((self.lambda)(args));
        }else{
            return Err("Argument number do not match");
        }
    }
    pub fn from_str(s: &str, al: &FnAlias) -> Result<Self, String>{
        match al.get_fn(s){
            Some(x) => Ok(Function::new(x.lambda, x.argc)),
            None => Err(format!("No such function: {}", s)),
        }
    }
}

enum Brc {
    Opening,
    Closing
}
pub struct Brace{
    brc_type: Brc
}
impl Brace {
    pub fn from_str(s: &str) -> Result <Self, String> {
        match s.trim() {
            "(" => Ok(Self { brc_type: (Brc::Opening) }),
            ")" => Ok(Self { brc_type: (Brc::Closing) }),
            _ => Err("Unknown brace type".to_string()),
        }
    }
}
