mod unit;
pub mod units;
pub use unit::Unit;
use unit::base_units::*;
pub use unit::base_units;


#[derive(Clone, Debug)]
pub struct Val{
    unit: Unit,
    magn: f64,
}
impl Val{
    pub fn new(magn:f64, unit: unit::Unit) -> Self{
        Val{magn, unit}
    }

    pub fn pow(self, p:f64) -> Self{
    let mut ret = Self::new(1., D);
    ret.unit = self.unit.pow(p);
    ret.magn = ret.magn.powf(p);
    ret
    }

    pub fn same_unit(other: &Val, precision: Option<f64>) -> bool{
        let precisionf;
        match precision {
            Some(x) => precisionf = x,
            None => precisionf = 0.02,
        };
        true
    }
}

use std::ops;

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

