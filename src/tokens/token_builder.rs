use super::val::{ValOpts, base_units::*};
use super::associations::{ValAlias, FnAlias};
use super::{Function, Val, Brace, Operator};

pub struct Builder{
    val_opts: ValOpts,
    val_alias: ValAlias,
    func_alias: FnAlias,
}

impl Default for Builder {
    fn default() -> Self {
        Builder { val_opts: ValOpts::default(),
        val_alias: ValAlias::default(),
        func_alias: FnAlias::default(), }
    }
}

impl Builder{
    pub fn val_from_str(&self, s: &str) -> Result<Val, String>{
        Val::from_str(s, &self.val_alias)
    }
    pub fn op_from_str(&self, s: &str) -> Result<Operator, String>{
        Operator::from_str(s)
    }
    pub fn brace_from_str(&self, s: &str) -> Result<Brace, String>{
        Brace::from_str(s)
    }
    pub fn function_from_str(&self, s: &str) -> Result<Function, String>{
         Function::from_str(s, &self.func_alias)
    }
}
