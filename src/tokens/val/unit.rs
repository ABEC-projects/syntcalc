pub mod base_units{
    use super::Unit;
    pub const D  :Unit = Unit{dim:[0., 0., 0., 0., 0., 0., 0.]};
    pub const S  :Unit = Unit{dim:[1., 0., 0., 0., 0., 0., 0.]};
    pub const M  :Unit = Unit{dim:[0., 1., 0., 0., 0., 0., 0.]};
    pub const KG :Unit = Unit{dim:[0., 0., 1., 0., 0., 0., 0.]};
    pub const A  :Unit = Unit{dim:[0., 0., 0., 1., 0., 0., 0.]};
    pub const K  :Unit = Unit{dim:[0., 0., 0., 0., 1., 0., 0.]};
    pub const MOL:Unit = Unit{dim:[0., 0., 0., 0., 0., 1., 0.]};
    pub const CD :Unit = Unit{dim:[0., 0., 0., 0., 0., 0., 1.]};
}


/// # Struct that represents number's Unit
/// Essentially, it's a vector with 7 floats,
/// each representing power of one of the base units
/// ## E. g. mol/m^3 is 
/// ```rust
/// use syntcalc::tokens::val::unit::Unit;
/// Unit::new([0., 0., -3., 0., 0., 1., 0.]);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Unit{
    pub dim: [f64;7],
}

impl Unit {
    pub fn new(dim:[f64;7]) -> Unit{
        Unit{dim}
    }

    pub fn pow(self, p:f64) -> Self{
        let mut ret = self;
        for i in 0..7{
            ret.dim[i] *= p;
        }
        ret
    }

    pub fn same_unit(self, other: &Unit, precision: f64) -> bool{
        let mut flag = true;
        for i in 0..7 {
            flag = flag && self.dim[i] - precision/2. < other.dim[i] &&
                self.dim[i] + precision/2. > other.dim[i];  
        }
        flag
    }
    
}


impl std::cmp::PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.dim == other.dim
    }
}

use std::ops;

impl ops::Add for Unit{
    type Output = Option<Unit>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.dim == rhs.dim {
            Option::Some(self)
        }else{
            Option::None
        }
    }
}

impl ops::Sub for Unit{
    type Output = Option<Unit>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.dim == rhs.dim {
            Option::Some(self)
        }else{
            Option::None
        }
    }
}

impl ops::Mul for Unit{
    type Output = Unit;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = self;
        for i in 0..7{
            ret.dim[i] += rhs.dim[i];
        };
        ret
    }
}

impl ops::MulAssign for Unit {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..7{
            self.dim[i] += rhs.dim[i];
        }
    }
}

impl ops::Div for Unit{
    type Output = Unit;
    fn div(self, rhs: Self) -> Self::Output {
        let mut ret = self;
        for i in 0..7{
            ret.dim[i] -= rhs.dim[i];
        };
        ret
    }
}

impl ops::DivAssign for Unit {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..7{
            self.dim[i] -= rhs.dim[i];
        }
    }
}

