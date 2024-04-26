pub mod val;

pub use val::Val as Val;
pub enum Tokens{
    Op(Operator),
    Val(Val),
    Brc(String),
    Fn(Function),
}

pub fn tokenize(expr: String) -> Vec<Tokens>{
    let pv = Vec::new();
    
    return pv;
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
    fn new(op: &str) -> Self{
        Operator{op:Ops::Add}
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
