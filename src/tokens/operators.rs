use std::str::FromStr;

use super::{val::ValComputeError, Val};

pub trait Operator {
    fn compute(&self, lhs: Val, rhs: Val) -> Result<Val, ValComputeError>;
    fn from_str(s: &str) -> Result<Self, String> where Self: Sized;
    fn get_precedence(&self) -> u32;
}

/// A binary operator struct
enum BinOps {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

pub struct BinOperator {
    kind: BinOps,
    precedence: u32,
}
impl BinOperator {
    fn new(op: BinOps, precedence: u32) -> Self{
        BinOperator { kind: op, precedence }
    }
    pub fn compute(&self, lhs: Val, rhs: Val) -> Result<Val, ValComputeError> {
        use BinOps::*;
        match self.kind {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => Ok(lhs * rhs),
            Div => Ok(lhs / rhs),
            Pow => lhs.pow_val(&rhs),
            Mod => todo!(),
        }
    }
    pub fn from_str(s: &str) -> Result<Self, String> {
        use BinOps::*;
        match match s{
            "+" => Some((Add, 2)),
            "-" => Some((Sub, 2)),
            "*" => Some((Mul, 4)),
            "/" => Some((Div, 4)),
            "^" => Some((Pow, 6)),
            "**" => Some((Pow, 6)),
            "%" => Some((Mod, 6)),
            _ => None,
        }{
            Some((op, prec)) => Ok(Self::new(op, prec)),
            None => Err(format!("No such operator: '{s}'")),
        }
    }
    pub fn get_precedence(&self) -> u32{
        self.precedence
    }
}

impl FromStr for BinOperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl Operator for BinOperator{
    fn compute(&self, lhs: Val, rhs: Val) -> Result<Val, ValComputeError> {
        self.compute(lhs, rhs)
    }
    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_str(s)
    }
    fn get_precedence(&self) -> u32 {
        self.get_precedence()
    }
}

enum UnOps {
    Neg, 
    Fac,
}

pub struct UnOperator{
    op: UnOps,
    precedence: u32,
}

impl UnOperator{
    fn new(op: UnOps) -> Self{
        UnOperator{op, precedence:100}
    }
    pub fn from_str(s: &str) -> Result<Self, String> {
        use UnOps::*;
        match match s{
            "-" => Some(Neg),
            "!" => Some(Fac),
            _ => None,
        }{
            Some(op) => Ok(Self::new(op)),
            None => Err(format!("No such operator: '{s}'")),
        }
    }
    pub fn get_precedence(&self) -> u32{
        self.precedence
    }
}

impl FromStr for UnOperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl Operator for UnOperator{
    fn compute(&self, lhs: Val, rhs: Val) -> Result<Val, ValComputeError> {
       self.compute(lhs, rhs)
    }
    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_str(s)
    }
    fn get_precedence(&self) -> u32 {
        self.get_precedence()
    }
}
