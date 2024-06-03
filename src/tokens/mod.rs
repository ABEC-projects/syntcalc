pub mod val;
pub mod token_builder;
pub mod associations;

mod operators;

use std::str::FromStr;

use val::ValComputeError;
pub use val::Val as Val;
pub use operators::*;



#[derive(Clone, Copy)]
pub struct Function{
    lambda: fn(Vec<Val>) -> Val,
    argc: u32,
}
use self::associations::FnAlias;
impl Function{
    pub fn new(lambda: fn(Vec<Val>) -> Val,  argc: u32) -> Self{
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
