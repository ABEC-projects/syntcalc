use std::collections::HashMap;
use super::unit::base_units::*;
use super::Val;

type Map = HashMap<String, Val>;

pub struct Associations{
    map: Map,
}

impl Associations {
    pub fn new() -> Self{
        let mut map: Map = HashMap::new();
        map.insert(String::from("km"), Val { unit: (M), magn: (1000.) });
        map.insert(String::from("g"), Val { unit: (KG), magn: (0.001) });
        map.insert(String::from("min"), Val { unit: (S), magn: (60.) });
        map.insert(String::from("ms"), Val { unit: (S), magn: (0.001) });
        map.insert(String::from("J"), Val { unit: (KG*M.pow(2.)/M.pow(2.)), magn: (1.) });
        map.insert(String::from("min"), Val { unit: (D), magn: (3.14159265359) });

        Associations{map}
    }
    pub fn get_unit(&self, key: &String) -> Val{
        return self.map[key];
    }
    pub fn get_map(&self) -> Map{
        self.map.clone()
    }
    pub fn insert(& mut self, key: String, value: Val){
        self.map.insert(key, value);
    }
}
