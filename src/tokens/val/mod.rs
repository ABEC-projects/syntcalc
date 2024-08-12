pub mod unit;
mod errors;

pub use unit::base_units;
pub use unit::Unit;
pub use errors::ValComputeError;
pub use errors::ErrorType as ValComputeErrorType;


use unit::base_units::*;



#[derive(Debug, Clone)]
pub struct ValOpts{
    cmp_epsilon: f64,
}

impl ValOpts{
    pub fn new() -> Self{
        ValOpts{..Default::default()}
    }
    pub fn set_cmp_epsilon(&mut self, cmp_epsilon: f64) -> &Self{
        self.cmp_epsilon = cmp_epsilon;
        self
    }
}

impl Default for ValOpts{ 
    fn default() -> Self {
        ValOpts{cmp_epsilon: 0.000001}
    }
}

/// Struct that represents a mathematical value with unit
#[derive(Clone, Debug)]
pub struct Val {
    unit: Unit,
    magn: f64,
    options: Arc<RefCell<ValOpts>>, 
}

impl Val {
    pub fn new (magn:f64, unit: Unit, options: Arc<RefCell<ValOpts>>) -> Self{
        Val{magn, unit, options}
    }

    pub fn get_opts(&self) -> Arc<RefCell<ValOpts>>{
        self.options.clone()
    }

    pub fn set_magnetude(&mut self, magn: f64){
        self.magn = magn;
    }

    pub fn pow(&self, p:f64) -> Self{
        let mut ret = self.clone();
        ret.unit = ret.unit.pow(p);
        ret.magn = ret.magn.powf(p);
        ret
    }

    pub fn pow_val(&self, p: &Val) -> Result<Self, ValComputeError>{
        let mut ret = self.clone();
        if !p.get_unit().same_unit( &base_units::D, self.options.borrow().cmp_epsilon){
            return Err(ValComputeError::new(
                    "Can not rise to a power with a unit".to_string(),
                    ValComputeErrorType::IncompatibleUnits));
        }
        let p = p.magn;
        ret.unit = ret.unit.pow(p);
        ret.magn = ret.magn.powf(p);
        Ok(ret)
    }


    pub fn same_unit(&self, other: &Val) -> bool{
        let precisionf = self.options.borrow().cmp_epsilon;
        self.unit.same_unit(&other.unit, precisionf)
    }

    pub fn get_unit(&self) -> Unit{
        self.unit
    }

    pub fn get_magnetude(&self) -> f64{
        self.magn
    }
    pub fn from_str  (s: &str, al: &ValAlias, options: Arc<RefCell<ValOpts>>) -> Result<Self, String> {
        use regex::Regex;
        let reg = 
            r"^(?<val>(?<neg>-)?(?<base>0[xbo])?(?<int>\d+)(\.(?<fract>\d+))?([Ee](?<exp>-?\d+))?)?(?<unit>\w+)?";
        let regex_val = Regex::new(reg).unwrap();
        let Some(caps) = regex_val.captures(s) else {return Err("Wrong value format!".to_string())};
        if caps.name("val").is_some(){
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
                    s.as_str().replace('-', "").parse::<i32>().unwrap() *
                        (if  s.as_str().contains('-') {-1} else {1})
                },
                None => 0,
            };
            let neg: f64 = match caps.name("neg"){
                Some(s) => if s.as_str().contains('-') {-1.} else {1.},
                None => 1.,
            };
            
            let mut magn = 0.;
            magn += neg*int_part as f64;
            magn +=  neg*(fract_part as f64) /
                ((base as f64).powi(fract_part_len));
            magn *= (base as f64).powi(exponent_part);

            let mut ret = Val::new(magn, D, options.clone());
            ret *= match caps.name("unit"){
                Some(s) => {match al.get_val(s.as_str()){
                    Some(v) => v,
                    None => return Err(format!("No {} found", s.as_str())),
                }},
                None => Self::new(1., D, options.clone()),
            };
            Ok(ret)
        }else{
            Err("No number found in the string".to_string())
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.magn)
    }
}

use std::cell::RefCell;
use std::fmt::Display;
use std::sync::Arc;
use std::ops;

use super::associations::ValAlias;

impl ops::Add for Val{
    type Output = Result<Self, ValComputeError>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = self;

        if ret.unit == rhs.unit {
            ret.magn += rhs.magn;
        }else{
            return Err(ValComputeError::new(
                "Units should be the same for addition".to_string(),
                ValComputeErrorType::IncompatibleUnits
            ));
        }

        Ok(ret)
    }
}

impl ops::Sub for Val{
    type Output = Result<Self, ValComputeError>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = self;

        if ret.unit == rhs.unit {
            ret.magn -= rhs.magn;
        }else{
            return Err(ValComputeError::new(
                "Units should be the same for addition".to_string(),
                ValComputeErrorType::IncompatibleUnits
                    ));
        }
        Ok(ret)
    }
}

impl ops::Neg for Val{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut ret = self;
        ret.magn = -ret.magn;
        ret
    }
}

impl ops::Mul for Val {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = self;

        ret.magn *= rhs.magn;
        ret.unit *= rhs.unit;
        ret
    }
}

impl ops::MulAssign for Val  {
    fn mul_assign(&mut self, rhs: Self) {
        self.magn *= rhs.magn;
        self.unit *= rhs.unit;
    }
}

impl ops::Div for Val  {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut ret = self;

        ret.magn /= rhs.magn;
        ret.unit /= rhs.unit;
        ret
    }
}

impl ops::DivAssign for Val  {
    fn div_assign(&mut self, rhs: Self) {
        self.magn /= rhs.magn;
        self.unit /= rhs.unit;
    }
}

use std::cmp;

impl cmp::PartialEq for Val  {
    fn eq(&self, other: &Self) -> bool {
        return (self.magn.abs()-other.magn.abs()).abs() < self.options.borrow().cmp_epsilon 
            && self.same_unit(other);
    }
}

impl cmp::PartialOrd for Val  {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.same_unit(other) {
            Some(self.magn.partial_cmp(&other.magn).unwrap())
        }else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_math() {
        let opts = Arc::new(RefCell::new(ValOpts::default()));
        let a = Val::new(1., D, opts.clone());
        let b = Val::new(1., KG, opts.clone());
        assert_eq!((a.clone()+a.clone()).unwrap(), Val::new(2., D, opts.clone()));
        assert_eq!(b.clone().pow(2.), Val::new(1., KG.pow(2.), opts.clone()));
    }
}
