pub mod val;
pub mod token_builder;
pub mod associations;

mod operators;

use std::sync::Arc;

use val::ValComputeError;
pub use val::Val as Val;
pub use operators::*;


#[derive(Clone)]
pub struct Function{
    lambda: Arc<dyn Fn(Vec<Val>) -> Result<Val, ValComputeError>>,
    argc: u32,
}


use self::associations::FnAlias;
impl Function{
    pub fn new (lambda: Arc<dyn Fn(Vec<Val>) -> Result<Val, ValComputeError>>,  argc: u32) -> Self{
        Function{lambda, argc}
    }
    pub fn compute (&self, args: Vec<Val>) -> Result<Val, String> {
        if args.len() ==  self.argc as usize{
            (self.lambda)(args).map_err(|x| x.to_string())
        }else{
            Err(format!("Argument number do not match.\nExpected: {}, found: {}", self.argc, args.len()))
        }
    }
    pub fn from_str(s: &str, al: &FnAlias) -> Result<Self, String>{
        match al.get_fn(s){
            Some(x) => Ok(Function::new(x.lambda, x.argc)),
            None => Err(format!("No such function: {}", s)),
        }
    }
}
