use std::collections::HashMap;
use super::{Function, Val};
use super::val::base_units::*;

type Map = HashMap<String, Val>;

pub struct ValAlias{
    map: Map,
}

impl Default for ValAlias {
    fn default() -> Self {
        let mut map: Map = HashMap::new();
        map.insert(String::from("km"), Val::new(1000., M));
        map.insert(String::from("g"), Val::new(0.001, KG));
        map.insert(String::from("kg"), Val::new(1., KG));
        map.insert(String::from("min"), Val::new(60., S));
        map.insert(String::from("ms"), Val::new(0.001, S));
        map.insert(String::from("s"), Val::new(1., S));
        map.insert(String::from("J"), Val::new(1., KG*M.pow(2.)/S.pow(2.)));
        map.insert(String::from("W"), Val::new(1., KG*M.pow(2.)/S.pow(3.)));
        map.insert(String::from("pi"), Val::new(3.141592653589793238462643383279502, D));

        ValAlias{map}
    }
}

impl ValAlias {
    pub fn new() -> Self{
        let mut map: Map = HashMap::new();
        map.insert(String::from("km"), Val::new(1000., M));
        map.insert(String::from("g"), Val::new(0.001, KG));
        map.insert(String::from("min"), Val::new(60., S));
        map.insert(String::from("ms"), Val::new(0.001, S));
        map.insert(String::from("s"), Val::new(1., S));
        map.insert(String::from("J"), Val::new(1., KG*M.pow(2.)/S.pow(2.)));
        map.insert(String::from("pi"), Val::new(3.14159265359, D));
        map.insert(String::from("W"), Val::new(1., KG*M.pow(2.)/S.pow(3.)));

        ValAlias{map}
    }
    pub fn get_val(&self, key: &str) -> Option<Val>{
        self.map.get(key).cloned()

    }
    pub fn get_map(&self) -> &Map{
        &self.map
    }
    pub fn set_map(&mut self, map: Map) -> &Self{
        self.map = map;
        self
    }
    pub fn add_alias(&mut self, key: String, value: Val){
        self.map.insert(key, value);
    }
}

type FnMap = HashMap<String, Function>;
pub struct FnAlias{
    map: FnMap,
}

impl Default for FnAlias {
    fn default() -> Self {
        let mut map = FnMap::new();
        map.insert("sin".to_string(), Function { lambda: 
            |x|{Val::new(x[0].get_magnetude().sin(), x[0].get_unit())},
            argc: (1) });
        map.insert("arcsin".to_string(), Function { lambda: 
            |x|{Val::new(x[0].get_magnetude().asin(), x[0].get_unit())},
            argc: (1) });
        Self {map}
    }
}

impl FnAlias {
    pub fn new() -> Self{
        Self { map: FnMap::new() }
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
