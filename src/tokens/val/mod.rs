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
pub struct Val <'a>{
    unit: Unit,
    magn: f64,
    options: &'a ValOpts,
}

impl <'b> Val <'b>{
    pub fn new <'a: 'b> (magn:f64, unit: Unit, options: &'a ValOpts) -> Self{
        Val{magn, unit, options}
    }

    pub fn get_opts(&self) -> &ValOpts{
        self.options
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
        if !p.get_unit().same_unit( &base_units::D, self.options.cmp_epsilon){
            return Err(ValComputeError::new(
                    "Can not rise to a power with a unit".to_string(),
                    ValComputeErrorType::IncompatibleUnits));
        }
        let p = p.magn;
        ret.unit = ret.unit.pow(p);
        ret.magn = ret.magn.powf(p);
        Ok(ret)
    }

    pub fn factorial(&self) -> Self{
        let mut ret = self.clone();
        todo!();
        ret
    }

    pub fn same_unit(&self, other: &Val) -> bool{
        let precisionf = self.options.cmp_epsilon;
        return self.unit.same_unit(&other.unit, precisionf);
    }

    pub fn get_unit(&self) -> Unit{
        self.unit
    }

    pub fn get_magnetude(&self) -> f64{
        self.magn
    }
    pub fn from_str <'a: 'b> (s: &str, al: &'a ValAlias, options: &'a ValOpts) -> Result<Self, String> {
        use regex::Regex;
        let reg = 
            r"^(?<val>(?<neg>-)?(?<base>0[xbo])?(?<int>\d+)(\.(?<fract>\d+))?([Ee](?<exp>-?\d+))?)?(?<unit>\w+)?";
        let regex_val = Regex::new(reg).unwrap();
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
                    s.as_str().replace("-", "").parse::<i32>().unwrap() *
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

            let mut ret = Val::new(magn, D, options);
            ret *= match caps.name("unit"){
                Some(s) => {match al.get_val(s.as_str()){
                    Some(v) => v,
                    None => return Err(format!("No {} found", s.as_str())),
                }},
                None => Self::new(1., D, options),
            };
            return Ok(ret);
        }else{
            return Err("No number found in the string".to_string());
        }
    }
}

impl Display for Val <'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo:{}{:?}", self.magn, self.unit)
    }
}

use std::fmt::Display;
use std::{ops, str::FromStr};

use super::associations::ValAlias;

impl ops::Add for Val<'_>{
    type Output = Result<Self, ValComputeError>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        if self.unit == rhs.unit {
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

impl ops::Sub for Val<'_>{
    type Output = Result<Self, ValComputeError>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        if self.unit == rhs.unit {
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

impl ops::Neg for Val<'_>{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut ret = self.clone();
        ret.magn = -ret.magn;
        ret
    }
}

impl ops::Mul for Val<'_> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        ret.magn *= rhs.magn;
        ret.unit *= rhs.unit;
        ret
    }
}

impl ops::MulAssign for Val <'_> {
    fn mul_assign(&mut self, rhs: Self) {
        self.magn *= rhs.magn;
        self.unit *= rhs.unit;
    }
}

impl ops::Div for Val <'_> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut ret = self.clone();

        ret.magn /= rhs.magn;
        ret.unit /= rhs.unit;
        ret
    }
}

impl ops::DivAssign for Val <'_> {
    fn div_assign(&mut self, rhs: Self) {
        self.magn /= rhs.magn;
        self.unit /= rhs.unit;
    }
}

use std::cmp;

impl cmp::PartialEq for Val <'_> {
    fn eq(&self, other: &Self) -> bool {
        return (
            self.magn.abs()-other.magn.abs()).abs() < self.options.cmp_epsilon &&
            self.same_unit(&other
        );
    }
}

impl cmp::PartialOrd for Val <'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.same_unit(&other) {
            return Some(self.magn.partial_cmp(&other.magn).unwrap());
        }else{
            return None;
        }
    }
}
