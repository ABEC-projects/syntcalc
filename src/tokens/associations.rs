use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use super::{Function, Val};
use super::val::{base_units::*, ValComputeError, ValComputeErrorType, ValOpts};


pub struct ValAlias{
    map: HashMap<String, Val>,
    valopts: Arc<RefCell<ValOpts>>, 
}

impl ValAlias {
    pub fn new (valopts: Arc<RefCell<ValOpts>>) -> Self{
        let map: HashMap<String, Val> = HashMap::new();
        ValAlias{map, valopts}
    }
    pub fn insert_default (&mut self) -> &Self{
        self.map.insert(String::from("km"), Val::new(1000., M, self.valopts.clone()));
        self.map.insert(String::from("g"), Val::new(0.001, KG, self.valopts.clone()));
        self.map.insert(String::from("kg"), Val::new(1., KG, self.valopts.clone()));
        self.map.insert(String::from("min"), Val::new(60., S, self.valopts.clone()));
        self.map.insert(String::from("ms"), Val::new(0.001, S, self.valopts.clone()));
        self.map.insert(String::from("s"), Val::new(1., S, self.valopts.clone()));
        self.map.insert(String::from("J"), Val::new(1., KG*M.pow(2.)/S.pow(2.), self.valopts.clone()));
        self.map.insert(String::from("W"), Val::new(1., KG*M.pow(2.)/S.pow(3.), self.valopts.clone()));
        self.map.insert(String::from("pi"), Val::new(3.141592653589793238462643383279502, D, self.valopts.clone()));

        self
    }
    pub fn get_val(&self, key: &str) -> Option<Val>{
        self.map.get(key).cloned()

    }
    pub fn get_map (&self) -> &HashMap<String, Val>{
        &self.map
    }
    pub fn set_map  (&mut self, map: HashMap<String, Val>) -> &Self{
        self.map = map;
        self
    }
    pub fn add_alias (&mut self, key: String, value: Val){
        self.map.insert(key, value);
    }
}

type FnMap = HashMap<String, Function>;
pub struct FnAlias {
    map: FnMap,
    valopts: Arc<RefCell<ValOpts>>,
}


impl  FnAlias {
    pub fn new  (valopts: Arc<RefCell<ValOpts>>) -> Self{
        Self { map: FnMap::new(), valopts}
    }
    pub fn insert_default(&mut self) -> &Self{
        self.map.insert( "ln".to_string(), Function { lambda:
            |x|{
                if x[0].get_magnetude() < 0.{
                    return Err(ValComputeError::new(
                            "Can not take the logarithm of a negative number".to_string(),
                            ValComputeErrorType::Other));
                }
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().ln());
                Ok(ret)
            },
            argc: 1});
        
        self.map.insert("sin".to_string(), Function {
            lambda: |x| {
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().sin());
                Ok(ret)
            },
            argc: 1
        });
        self.map.insert("cos".to_string(), Function {
            lambda: |x| {
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().cos());
                Ok(ret)
            },
            argc: 1
        });

        self
    }
    pub fn get_fn(&self, key: &str) -> Option<Function>{
        self.map.get(key).cloned()
    }
    pub fn get_map(&self) -> &FnMap{
        &self.map
    }
    pub fn set_map(&mut self, map: FnMap) -> &Self{
        self.map = map;
        self
    }
    pub fn add_alias(&mut self, key: String, value: Function){
        self.map.insert(key, value);
    }
}
