
use std::cell::RefCell;
use std::sync::Arc;

use super::val::ValOpts;
use super::associations::{ValAlias, FnAlias};
use super::{BinOperator, Function, UnOperator, Val};

#[derive(Clone, Default)]
pub struct Builder {
    pub val_opts: Arc<RefCell<ValOpts>>,
    pub val_alias: Arc<RefCell<ValAlias>>,
    pub func_alias: Arc<RefCell<FnAlias>>,
    pub local_val_alias: ValAlias,
}


impl Builder {
    pub fn new () -> Self{
        let val_opts = Arc::new(RefCell::new(ValOpts::default()));
        Builder{val_opts: val_opts.clone(),
                val_alias: Arc::new(RefCell::new(ValAlias::new(val_opts.clone()))),
                func_alias: Arc::new(RefCell::new(FnAlias::new())),
                local_val_alias: ValAlias::new(val_opts.clone())}
    }
    pub fn val_from_str(&self, s: &str) -> Result<Val, String>{
         Val::from_str(s, &self.val_alias.borrow(), self.val_opts.clone())
    }
    pub fn bin_op_from_str(&self, s: &str) -> Result<BinOperator, String>{
        BinOperator::from_str(s)
    }
    pub fn un_op_from_str(&self, s: &str) -> Result<UnOperator, String>{
        UnOperator::from_str(s)
    }
    pub fn function_from_str(&self, s: &str) -> Result<Function, String>{
         Function::from_str(s, &self.func_alias.borrow())
    }
    pub fn get_var_val(&self, s: &str) -> Result<Val, String>{
        let local = self.local_val_alias.get_val(s);
        match local{
            Some(v) => Ok(v),
            None => self.val_alias.borrow().get_val(s).ok_or_else(|| format!("Variable {} not found", s)),
        }
    }
    pub fn insert_defaults(&mut self){
        self.val_alias.borrow_mut().insert_default();
        self.func_alias.borrow_mut().insert_default();
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_if_can_be_mutable(){
        let mut b = Builder::new();
        b.insert_defaults();
        b.val_opts.borrow_mut().set_cmp_epsilon(0.1);
        b.val_alias.borrow_mut().add_alias("a".to_string(), b.val_from_str("1.0").unwrap());
        b.func_alias.borrow_mut().add_alias("b".to_string(), b.func_alias.borrow_mut().get_fn("sin").unwrap());
    }
}
