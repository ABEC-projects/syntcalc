mod unit;
pub mod units;
pub use unit::Unit;
use unit::base_units::*;
pub use unit::base_units;

#[derive(Debug, Clone)]
pub struct ValOpts{
    
}

impl Default for ValOpts{
    fn default() -> Self {
        ValOpts{}
    }
}

#[derive(Clone, Debug)]
pub struct Val{
    unit: Unit,
    magn: f64,
    pub options: ValOpts
}

impl Default for Val{
    fn default() -> Self {
        Val { unit: (D), magn: (1.), options: (ValOpts::default())}
    }
}

impl Val{
    pub fn new(magn:f64, unit: unit::Unit) -> Self{
        Val{magn, unit, options: ValOpts::default()}
    }

    pub fn set_options(&mut self, options: ValOpts) -> &Self{
        self.options = options;
        self
    }

    pub fn pow(&self, p:f64) -> Self{
    let mut ret = Self::new(1., D);
    ret.unit = self.unit.pow(p);
    ret.magn = ret.magn.powf(p);
    ret
    }

    pub fn same_unit(&self, other: &Val, precision: Option<f64>) -> bool{
        let precisionf;
        match precision {
            Some(x) => precisionf = x,
            None => precisionf = 0.02,
        };
        return self.unit.same_unit(&other.unit, precisionf);
    }

    pub fn get_unit(&self) -> Unit{
        self.unit
    }
    

    pub fn get_magnetude(&self) -> f64{
        self.magn
    }
    pub fn from_str(s: &str) -> Result<Self, String> {
        use regex::Regex;
        // let regex_val = Regex::new(r"^(?<base>0[xbo])?(?<val>(?<int>-?\d+)(\.(?<fract>\d+)(E(?<exp>-?\d+))?)?)$").unwrap();
        let regex_val = Regex::new(r"^(?<val>(?<neg>-)?(?<base>0[xbo])?(?<int>\d+)(\.(?<fract>\d+))?([Ee](?<exp>-?\d+))?)$").unwrap();
        let Some(caps) = regex_val.captures(s) else {return Err("Wrong value format!".to_string())};
        if let Some(_) = caps.name("val"){
            let base:u32 = match caps.name("base"){
                Some(s) => {
                    match s.as_str(){
                        "0b" => 2,
                        "0o" => 8,
                        "0x" => 16,
                        _ => unreachable!(),
                    }
                },
                None => 10,
            };
            let to_u64_base = |s:&str, base: u32|{
                let len = s.len() as u32;
                let mut ret:u64 = 0;
                for i in 0..len{
                    ret += (base as u64).pow(len-1-i)*
                        s.chars().nth(i as usize).unwrap().to_string().parse::<u64>().unwrap();
                }
                ret 
            };
            let int_part: u64 = match caps.name("int") {
                Some(s) => to_u64_base(s.as_str(), base),
                None => unreachable!(),
            };
            let fract_part: f64 = match caps.name("fract"){
                Some(s) => to_u64_base(s.as_str(), base) as f64,
                None => 0.,
            };
            let fract_part_len: i32 = match caps.name("fract"){
                Some(s) => s.as_str().len() as i32,
                None => 0
            };
            let exponent_part: i32 = match caps.name("exp"){
                Some(s) => {
                    s.as_str().replace("-", "").parse::<i32>().unwrap()*
                        (if  s.as_str().contains('-') {-1} else {1})
                },
                None => 0,
            };
            let neg: f64 = match caps.name("neg"){
                Some(s) => if s.as_str().contains("-") {-1.} else {1.},
                None => 1.,
            };

            let mut magn = 0.;
            magn += neg*int_part as f64;
            magn +=  neg*(fract_part as f64) /
                ((base as f64).powi(fract_part_len));
            magn *= (base as f64).powi(exponent_part);

            return Ok(Val::new(magn, D));
        }else{
            return Err("No value found in the string".to_string());
        }
    }
}

impl FromStr for Val{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

use std::{ops, str::FromStr};

impl ops::Add for Val{
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        if self.unit == rhs.unit {
            ret.magn += rhs.magn;
        }else{
            return Option::None;
        }

        Some(ret)
    }
}

impl ops::Sub for Val{
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        if self.unit == rhs.unit {
            ret.magn += rhs.magn;
        }else{
            return Option::None;
        }

        Some(ret)
    }
}

impl ops::Mul for Val {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        ret.magn *= rhs.magn;
        ret.unit *= rhs.unit;
        ret
    }
}

impl ops::MulAssign for Val {
    fn mul_assign(&mut self, rhs: Self) {
        self.magn *= rhs.magn;
        self.unit *= rhs.unit;
    }
}

impl ops::Div for Val {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        ret.magn /= rhs.magn;
        ret.unit /= rhs.unit;
        ret
    }
}

impl ops::DivAssign for Val {
    fn div_assign(&mut self, rhs: Self) {
        self.magn /= rhs.magn;
        self.unit /= rhs.unit;
    }
}

impl std::cmp::PartialEq for Val{
    fn eq(&self, other: &Self) -> bool {
        return self.magn == other.magn && self.unit == other.unit;
    }
}

