use super::tokens::{Val, BinOperator, UnOperator, Operator Function};
use super::tokens::token_builder::Builder;
use pest::{self, Parser};
use pest_derive::Parser;
use std::collections::{VecDeque};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug)]
pub struct ParseError{
    desc: String,
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}
impl Error for ParseError{}

#[derive(Clone)]
enum Expr {
    Val(Val),
    Prefix(BinOperator),
    Infix(UnOperator),
    Postfixfix(UnOperator),
}
enum Tree {
    Infix(BinOperator, Box<Tree>, Box<Tree>),
    Prefix(BinOperator, Box<Tree>),
    Postfixfix(BinOperator, Box<Tree>),
    Val(Val),
}

#[derive(Default)]
pub struct SyntCalc{
    token_builder: Builder,
}

/// Main class for synthcalc crate.
/// Used to parse expressions and evaluate them with eval_str() function.
impl SyntCalc {
    pub fn eval_str(&self, expr: &str) -> Result<Val, ParseError>{
        let parsed = match MathParser::parse(Rule::file, expr){
            Ok(parsed) => parsed,
            Err(e) => return Err(ParseError{desc: format!("Parse error: {}", e)}),
        };
        self.eval_parsed(parsed)
    }

    fn eval_parsed(&self, parsed: pest::iterators::Pairs<Rule>) -> Result<Val, ParseError> {
        let mut val_op_sequence = Vec::new();
        for pair in parsed {
            match pair.as_rule() {
                Rule::number => val_op_sequence.push(
                    Expr::Val(self.token_builder.val_from_str(pair.as_str()).unwrap())),
                Rule::infix => val_op_sequence.push(
                    Expr::Infix(UnOperator::from_str(pair.as_str()).unwrap())),
                Rule::func => val_op_sequence.push(
                    Expr::Val(self.token_builder.function_from_str(pair.as_str()).unwrap().compute(
                            self.get_args_from_func_pair(&pair).unwrap()).unwrap())),
                Rule::expr => val_op_sequence.push(
                    Expr::Val(self.eval_parsed(pair.into_inner())?)),
                _ => todo!("unimplemented rule: {:?}", pair.as_rule()),
            }
        }
        let tree = Self::shounting_yard(&val_op_sequence)?;
        todo!()
    }

    /// makes operation tree considering operators' precedence
    fn shounting_yard(val_op_sequence: &Vec<Expr>) -> Result<Tree, ParseError> {
        let mut val_op_sequence = VecDeque::from((*val_op_sequence).clone());
        let mut reversed_polish: Vec<Expr> = Vec::new();
        let mut op_stack: Vec<Box<dyn Operator>> = Vec::new();

        for val_op in val_op_sequence {
            match val_op {
                Expr::Val(_) => reversed_polish.push(val_op),
                Expr::Infix(op) => {
                    if op.get_precedence() > op_stack.last().unwrap().deref().get_precedence() {
                        reversed_polish.push(op_stack.pop().unwrap());
                        op_stack.push(Box::new(op));
                    }
                }
                _ => todo!(),
            }
        }
        todo!()
    }

    fn compute_tree_branch(branch: &Tree) -> Result<Val, ParseError> {
        match branch {
            Tree::Val(val) => Ok(val.clone()),
            Tree::Infix(op, lhs, rhs) => {
                let l_val = Self::compute_tree_branch(lhs)?;
                let r_val = Self::compute_tree_branch(rhs)?;
                todo!()
            }
            Tree::Prefix(op, rhs) => {
                let r_val = Self::compute_tree_branch(rhs)?;
                todo!()
            }
            Tree::Postfixfix(op, lhs) => {
                let l_val = Self::compute_tree_branch(lhs)?;
                todo!()
            }
        }
    }

    fn get_args_from_func_pair(&self, pair: &pest::iterators::Pair<Rule>) -> Option<Vec<Val>> {
        match pair.as_rule() {
            Rule::func => {
                let mut args = Vec::new();
                for arg in pair.clone().into_inner() {
                    let arg_val = self.eval_parsed(arg.into_inner());
                    match arg_val {
                        Ok(val) => args.push(val),
                        Err(_) => return None,
                    }
                }
                return Some(args);
            },
            _ => return None,
        }
    }
}





#[derive(Parser)]
#[grammar = "expr_parser.pest"]
pub struct MathParser{}

