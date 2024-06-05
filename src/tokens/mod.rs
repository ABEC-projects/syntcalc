pub mod val;
pub mod token_builder;
pub mod associations;

mod operators;

use val::ValComputeError;
pub use val::Val as Val;
pub use operators::*;



#[derive(Clone, Copy)]
pub struct Function{
    lambda: for <'a> fn(Vec<Val<'a>>) -> Result<Val<'a>, ValComputeError>,
    argc: u32,
}
use self::associations::FnAlias;
impl Function{
    pub fn new (lambda: for <'a> fn(Vec<Val<'a>>) -> Result<Val<'a>, ValComputeError>,  argc: u32) -> Self{
        Function{lambda, argc}
    }
    pub fn compute <'a> (&self, args: Vec<Val<'a>>) -> Result<Val<'a>, String> {
        if args.len() ==  self.argc as usize{
            return (self.lambda)(args).map_err(|x| x.to_string());
        }else{
            return Err("Argument number do not match".to_string());
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
