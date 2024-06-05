
use super::val::{ValOpts, base_units::*};
use super::associations::{ValAlias, FnAlias};
use super::{BinOperator, Brace, Function, UnOperator, Val};

pub struct Builder <'a>{
    val_opts: &'a ValOpts,
    val_alias: ValAlias<'a>,
    func_alias: FnAlias<'a>,
}


impl <'b> Builder <'b>{
    pub fn new (val_opts: &'b ValOpts) -> Self{
        Builder{val_opts, val_alias: ValAlias::new(&val_opts), func_alias: FnAlias::new(&val_opts)}
    }
    pub fn val_from_str(&self, s: &str) -> Result<Val, String>{
         Val::from_str(s, &self.val_alias, &self.val_opts)
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
    pub fn get_var_val(&self, s: &str) -> Result<Val, String>{
        self.val_alias.get_val(s).ok_or_else(|| format!("Variable {} not found", s))
    }
}
