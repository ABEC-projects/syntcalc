use std::{fmt::Display, str::FromStr};

use super::{val::ValComputeError, Val};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
    Non,
}


pub trait Operator  where Self: Clone{
    fn from_str(s: &str) -> Result<Self, String> where Self: Sized;
    fn get_precedence(&self) -> u32;
    fn get_associativity(&self) -> Associativity;
}

/// A binary operator struct
#[derive(Clone, Copy, Debug)]
enum BinOps {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Clone, Copy, Debug)]
pub struct BinOperator {
    kind: BinOps,
    precedence: u32,
    associativity: Associativity,
}
impl BinOperator {
    fn new(op: BinOps, precedence: u32, associativity: Associativity) -> Self{
        BinOperator { kind: op, precedence, associativity }
    }
    pub fn compute (&self, lhs: Val, rhs: Val) -> Result<Val, ValComputeError> {
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
        use Associativity::*;
        match match s{
            "+" => Some((Add, 2, Left)),
            "-" => Some((Sub, 2, Left)),
            "*" => Some((Mul, 4, Left)),
            "/" => Some((Div, 4, Left)),
            "^" => Some((Pow, 6, Right)),
            "**" => Some((Pow, 6, Right)),
            "%" => Some((Mod, 6, Left)),
            _ => None,
        }{
            Some((op, prec, ass)) => Ok(Self::new(op, prec, ass)),
            None => Err(format!("No such operator: '{s}'")),
        }
    }
    pub fn get_precedence(&self) -> u32{
        self.precedence
    }
    pub fn get_associativity(&self) -> Associativity{
        self.associativity
    }
}

impl Display for BinOperator{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BinOps::*;
        let name = match self.kind{
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Pow => "^",
            Mod => "%",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for BinOperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl Operator for BinOperator{
    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_str(s)
    }
    fn get_precedence(&self) -> u32 {
        self.get_precedence()
    }
    fn get_associativity(&self) -> Associativity {
        self.get_associativity()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnOps {
    Neg, 
}

#[derive(Clone, Copy, Debug)]
pub struct UnOperator{
    kind: UnOps,
    precedence: u32,
    associativity: Associativity,
}

impl UnOperator{
    fn new(kind: UnOps, precedence: u32, associativity: Associativity) -> Self{
        UnOperator{kind, precedence, associativity}
    }
    pub fn from_str(s: &str) -> Result<Self, String> {
        use UnOps::*;
        use Associativity::*;
        match match s{
            "-" => Some((Neg, 10, Right)),
            _ => None,
        }{
            Some((op, prec, ass)) => Ok(Self::new(op, prec, ass)),
            None => Err(format!("No such operator: '{s}'")),
        }
    }
    pub fn compute  (&self, val: Val  ) -> Result<Val , ValComputeError> {
        use UnOps::*;
        match self.kind {
            Neg => Ok(-val),
        }
    }
    pub fn get_precedence(&self) -> u32{
        self.precedence
    }
    pub fn get_op_type(&self) -> UnOps{
        self.kind
    }
    pub fn get_associativity(&self) -> Associativity{
        self.associativity
    }
}

impl Display for UnOperator{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.kind{
            UnOps::Neg => "-",
        };
        write!(f, "{}", name)
    }
}
impl FromStr for UnOperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl Operator for UnOperator{
    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_str(s)
    }
    fn get_precedence(&self) -> u32 {
        self.get_precedence()
    }
    fn get_associativity(&self) -> Associativity {
        self.get_associativity()
    }
}
