
use super::val::{ValOpts, base_units::*};
use super::associations::{ValAlias, FnAlias};
use super::{BinOperator, Brace, Function, UnOperator, Val};

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
    pub fn bin_op_from_str(&self, s: &str) -> Result<BinOperator, String>{
        BinOperator::from_str(s)
    }
    pub fn un_op_from_str(&self, s: &str) -> Result<UnOperator, String>{
        UnOperator::from_str(s)
    }
    pub fn function_from_str(&self, s: &str) -> Result<Function, String>{
         Function::from_str(s, &self.func_alias)
    }
}
