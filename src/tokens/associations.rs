use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use super::{val, Function, Val};
use super::val::{base_units::*, Unit, ValComputeError, ValComputeErrorType, ValOpts};

#[derive(Clone, Debug)]
pub(crate) struct UnitAlias{
    all_units: Vec<(val::unit::Unit, String)>
}

impl UnitAlias {
    pub(crate) fn get_name(&self, unit: &Unit, precision: f64) -> Option<String>{
        for (u, name) in &self.all_units {
            let mut flag = true;
            for i in 0..7 {
                flag = flag && (u.dim[i] - unit.dim[i]).abs() < precision as f64;
            }
            if flag {
                return Some(name.clone());
            }
        };
        None
    }

    pub(crate) fn append(&mut self, unit: Unit, name: String) {
        self.all_units.push((unit, name));
    }
    
    pub(crate) fn new() -> Self {
        Self { all_units: Vec::new() }
    }
}

impl Default for UnitAlias {
    fn default() -> Self {
        use val::unit::base_units::*;
        let vec = vec![
            (M, "m".to_owned()),
            (D, "".to_owned()),
            (KG, "kg".to_owned())
        ];
        Self { all_units: vec }
    }
}

#[derive(Clone)]
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
        self.map.insert(String::from("m"), Val::new(1., M, self.valopts.clone()));
        self.map.insert(String::from("km"), Val::new(1000., M, self.valopts.clone()));
        self.map.insert(String::from("g"), Val::new(0.001, KG, self.valopts.clone()));
        self.map.insert(String::from("kg"), Val::new(1., KG, self.valopts.clone()));
        self.map.insert(String::from("min"), Val::new(60., S, self.valopts.clone()));
        self.map.insert(String::from("ms"), Val::new(0.001, S, self.valopts.clone()));
        self.map.insert(String::from("s"), Val::new(1., S, self.valopts.clone()));
        self.map.insert(String::from("J"), Val::new(1., KG*M.pow(2.)/S.pow(2.), self.valopts.clone()));
        self.map.insert(String::from("W"), Val::new(1., KG*M.pow(2.)/S.pow(3.), self.valopts.clone()));
        self.map.insert(String::from("pi"), Val::new(std::f64::consts::PI, D, self.valopts.clone()));

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
#[derive(Clone)]
pub struct FnAlias {
    map: FnMap,
}

impl Default for FnAlias {
    fn default() -> Self {
        let mut ret = FnAlias::new();
        ret.insert_default();
        ret
    }
}


impl  FnAlias {
    pub fn new  () -> Self{
        Self { map: FnMap::new()}
    }
    pub fn insert_default(&mut self) -> &Self{
        self.map.insert( "ln".to_string(), Function { lambda: Arc::new(
            |x: Vec<Val>|{
                if x[0].get_magnetude() < 0.{
                    return Err(ValComputeError::new(
                            "Can not take a logarithm of a negative number".to_string(),
                            ValComputeErrorType::Other));
                }
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().ln());
                Ok(ret)
            }),
            argc: 1});
        
        self.map.insert("sin".to_string(), Function {
            lambda: 
            Arc::new( |x| {
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().sin());
                Ok(ret)
            }),
            argc: 1
        });
        self.map.insert("cos".to_string(), Function {
            lambda: 
            Arc::new(|x| {
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().cos());
                Ok(ret)
            }),
            argc: 1
        });
        self.map.insert("tan".to_string(), Function::new(
                Arc::new(|x|{
                    let mut ret = x[0].clone();
                    ret.set_magnetude(ret.get_magnetude().tan());
                    Ok(ret)
                }),1));
        self.map.insert("cot".to_string(), Function::new(
                Arc::new(|x|{
                    let mut ret = x[0].clone();
                    ret.set_magnetude(ret.get_magnetude().tan().recip());
                    Ok(ret)
                }),1));
        self.map.insert("arcsin".to_string(), Function::new(
                Arc::new(|x|{
                    let mut ret = x[0].clone();
                    ret.set_magnetude(ret.get_magnetude().asin());
                    Ok(ret)
                }),1));
        self.map.insert("arccos".to_string(), Function::new(
                Arc::new(|x|{
                    let mut ret = x[0].clone();
                    ret.set_magnetude(ret.get_magnetude().acos());
                    Ok(ret)
                }),1));
        self.map.insert("arctan".to_string(), Function{
            lambda: Arc::new(|x|{
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().atan());
                Ok(ret)
            }),
            argc: 1
        });
        self.map.insert("arccot".to_string(), Function{
            lambda: Arc::new(|x|{
                let mut ret = x[0].clone();
                ret.set_magnetude(-(ret.get_magnetude().atan()-std::f64::consts::PI/2.));
                Ok(ret)
            }),
            argc: 1
        });
        self.map.insert("abs".to_string() , Function::new(
            Arc::new(|x|{
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().abs());
                Ok(ret)
            }),1   
        ));
        self.map.insert("fract".to_string() , Function::new(
            Arc::new(|x|{
                let mut ret = x[0].clone();
                ret.set_magnetude(ret.get_magnetude().fract());
                Ok(ret)
            }),1   
        ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_alias(){
       let al = UnitAlias::default();
       assert_eq!(al.get_name(&M, 0.0001).unwrap(), "m");
       assert_eq!(al.get_name(&KG, 0.0001).unwrap(), "kg");
       assert_ne!(al.get_name(&D, 0.0001).unwrap(), "kg");
    }
}









