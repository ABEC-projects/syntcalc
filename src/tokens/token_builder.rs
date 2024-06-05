
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Arc;

use super::val::{ValOpts, base_units::*};
use super::associations::{ValAlias, FnAlias};
use super::{BinOperator, Brace, Function, UnOperator, Val};

#[derive(Clone)]
pub struct Builder {
    pub val_opts: Arc<RefCell<ValOpts>>,
    pub val_alias: ValAlias,
    pub func_alias: FnAlias,
}


impl Builder {
    pub fn new (val_opts: Arc<RefCell<ValOpts>>) -> Self{
        Builder{val_opts: val_opts.clone(),
                val_alias: ValAlias::new(val_opts.clone()),
                func_alias: FnAlias::new(val_opts.clone())}
    }
    pub fn val_from_str(&self, s: &str) -> Result<Val, String>{
         Val::from_str(s, &self.val_alias, self.val_opts.clone())
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
    pub fn insert_defaults(&mut self){
        self.val_alias.insert_default();
        self.func_alias.insert_default();
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_if_can_be_mutable(){
        let val_opts = ValOpts::default();
        let mut b = Builder::new(Arc::new(RefCell::new(val_opts)));
        b.insert_defaults();
        b.val_opts.borrow_mut().set_cmp_epsilon(0.1);
        b.val_alias.add_alias("a".to_string(), b.val_from_str("1.0").unwrap());
        b.func_alias.add_alias("b".to_string(), b.func_alias.get_fn("sin").unwrap());
    }
}
