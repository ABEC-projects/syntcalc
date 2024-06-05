use crate::tokens::val::{self, ValComputeError, ValComputeErrorType};
pub use crate::tokens::val::ValOpts;

use super::tokens::{Val, BinOperator, UnOperator, Function};
use super::tokens::token_builder::Builder;
use pest::{self, Parser};
use pest_derive::Parser;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

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
enum Expr  {
    Val(Val ),
    Prefix(UnOperator),
    Infix(BinOperator),
    Postfixfix(UnOperator),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self{
            Expr::Val(val) => format!("Val: {}", val), 
            Expr::Infix(op) => format!("Infix: {}", op),
            Expr::Prefix(op) => format!("Prefix: {}", op),
            Expr::Postfixfix(op) => format!("Postfixfix: {}", op),
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Copy, Clone)]
enum Op{
    Un(UnOperator),
    Bin(BinOperator),
}
impl Op {
    fn get_precedence(&self) -> u32{
        match self{
            Op::Un(op) => op.get_precedence(),
            Op::Bin(op) => op.get_precedence(),
        }
    }
    fn as_expr(self) -> Expr{
        use crate::tokens::UnOps;
        match self{
            Op::Un(op) => {
                match op.get_op_type(){
                    UnOps::Neg => Expr::Prefix(op),
                    UnOps::Fac => Expr::Postfixfix(op),
                }
            },
            Op::Bin(op) => Expr::Infix(op),
        }
    }
}

#[derive(Clone)]
pub struct SyntCalc {
    pub token_builder: Builder,
}

/// Main class for synthcalc crate.
/// Used to parse expressions and evaluate them with eval_str() function.
impl  SyntCalc  {
    pub fn new () -> Self{
        Self{
            token_builder: Builder::new(Arc::new(RefCell::new(ValOpts::default()))), //ValOpts::default()),
        }
    }
    pub fn eval_str(&mut self, expr: &str) -> Result<Val, ParseError>{
        let parsed = match MathParser::parse(Rule::file, expr){
            Ok(parsed) => parsed,
            Err(e) => return Err(ParseError{desc: format!("Parse error:\n{}", e)}),
        };
        self.eval_parsed(parsed)
    }

    fn eval_parsed(&mut self, parsed: pest::iterators::Pairs<Rule>) -> Result<Val, ParseError> {
        let mut val_op_sequence = Vec::new();
        for pair in parsed {
            // println!("pair: {}", pair);
            // println!("val_op_sequence: {:#?}\n", val_op_sequence);
            match pair.as_rule() {
                Rule::file => val_op_sequence.push(
                    Expr::Val(self.eval_parsed(pair.into_inner())?)),
                Rule::number => val_op_sequence.push(
                    Expr::Val(self.token_builder.val_from_str(pair.as_str()).unwrap())),
                Rule::infix => val_op_sequence.push(
                    Expr::Infix(BinOperator::from_str(pair.as_str()).unwrap())),
                Rule::func => val_op_sequence.push(
                    Expr::Val(self.token_builder.function_from_str(
                            pair.clone().into_inner().next().unwrap().as_str()).unwrap().compute(
                            self.get_args_from_func_pair(&pair).unwrap()).unwrap())),
                Rule::expr => val_op_sequence.push(
                    Expr::Val(self.eval_parsed(pair.into_inner())?)),
                Rule::var => val_op_sequence.push(
                    Expr::Val(match self.token_builder.get_var_val(pair.as_str()){
                        Ok(val) => val,
                        Err(e) => return Err(ParseError{desc: format!("Error in while processing variables: {}", e)}),
                    })),
                Rule::add => val_op_sequence.push(Expr::Infix(BinOperator::from_str("+").unwrap())),
                Rule::sub => val_op_sequence.push(Expr::Infix(BinOperator::from_str("-").unwrap())),
                Rule::mul => val_op_sequence.push(Expr::Infix(BinOperator::from_str("*").unwrap())),
                Rule::neg => val_op_sequence.push(Expr::Prefix(UnOperator::from_str("-").unwrap())),
                Rule::fac => val_op_sequence.push(Expr::Postfixfix(UnOperator::from_str("!").unwrap())),
                Rule::div => val_op_sequence.push(Expr::Infix(BinOperator::from_str("/").unwrap())),
                Rule::pow => val_op_sequence.push(Expr::Infix(BinOperator::from_str("^").unwrap())),
                Rule::ternary => {
                    let mut inner = pair.into_inner();
                    let lhs = self.eval_parsed(inner.next().unwrap().into_inner())?;
                    let cond_type = inner.next().unwrap();
                    let rhs = self.eval_parsed(inner.next().unwrap().into_inner())?;
                    let if_true = inner.next().unwrap();
                    let if_false = inner.next().unwrap();
                    let flag;
                    match cond_type.as_rule(){
                        Rule::greater => flag = lhs > rhs,
                        Rule::less => flag = lhs < rhs,
                        Rule::equal => flag = lhs == rhs,
                        Rule::greaterEqual => flag = lhs >= rhs,
                        Rule::lessEqual => flag = lhs <= rhs,
                        Rule::notEqual => flag = lhs != rhs,
                        _ => unreachable!(),
                    }
                    if flag{
                        val_op_sequence.push(Expr::Val(self.eval_parsed(if_true.into_inner())?));
                    }else{
                        val_op_sequence.push(Expr::Val(self.eval_parsed(if_false.into_inner())?));
                    }
                },
                Rule::add_var => {
                    let mut inner = pair.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    let val = self.eval_parsed(inner.next().unwrap().into_inner())?;
                    self.token_builder.val_alias.add_alias(name, val.clone());
                },
                Rule::add_function => {
                    let inner = pair.into_inner();
                    let mut names = Vec::new();
                    let mut body = None;
                    for pair in inner {
                        match pair.as_rule() {
                            Rule::name => {
                                names.push(pair);
                            },
                            Rule::expr => {
                                body = Some(pair);
                            },
                            _ => unreachable!(),
                        }
                    }

                    let body = body.unwrap().as_str().to_string();
                    let fn_name = names[0].as_str().to_string();
                    let argc = names.len() - 1;
                    let sc = self.clone();

                    let names: Vec<String> = names.iter().skip(1).map(
                        |name| name.as_str().to_string()).collect();

                    let lambda = move |vals: Vec<Val>|{
                        let arg_names = names.clone();
                        let body = body.clone();
                        let mut sc = sc.clone();
                        for i in 0..arg_names.len() {
                            sc.token_builder.val_alias.add_alias(arg_names[i].clone(), vals[i].clone())
                        }
                        return sc.eval_str(&body).map_err(|e| ValComputeError::new(e.desc, ValComputeErrorType::Other));
                    };

                    let func = Function::new(std::sync::Arc::new(lambda), argc as u32);

                    self.token_builder.func_alias.add_alias(fn_name, func);
                },
                Rule::EOI => break,
                _ => todo!("unimplemented rule: {:?}", pair.as_rule()),
            }
        }
        if val_op_sequence.is_empty() {
            return Err(ParseError{desc: "".to_string()});
        }
        let val_op_sequence = Self::shounting_yard(&val_op_sequence)?;
        Ok(Self::compute_expr_vec(&val_op_sequence)?)
    }

    fn get_args_from_func_pair(&mut self, pair: &pest::iterators::Pair<Rule>) -> Option<Vec<Val>> {
        match pair.as_rule() {
            Rule::func => {
                let mut args = Vec::new();
                for arg in pair.clone().into_inner() {
                    match arg.as_rule() {
                        Rule::args =>{
                            let arg_val = self.eval_parsed(arg.into_inner());
                            match arg_val {
                                Ok(val) => args.push(val),
                                Err(_) => return None,
                            }
                        }
                        Rule::name => (), 
                        _ => unreachable!(),
                    }
                }
                return Some(args);
            },
            _ => return None,
        }
    }
    /// makes operation tree considering operators' precedence
    fn shounting_yard (val_op_sequence: &Vec<Expr>) -> Result<Vec<Expr>, ParseError> {
        use crate::tokens::Associativity;
        let val_op_sequence = VecDeque::from((*val_op_sequence).clone());
        let mut reversed_polish: Vec<Expr> = Vec::new();
        let mut op_stack: Vec<Op> = Vec::new();

        // println!("val_op_sequence: {:#?}", val_op_sequence);

        for val_op in val_op_sequence {
            match val_op {
                Expr::Val(_) => reversed_polish.push(val_op),
                Expr::Infix(op) => {
                    while op_stack.len() > 0 {
                        if let Some(last_op) = op_stack.last() {
                            if op.get_precedence() < last_op.get_precedence() || 
                                op.get_precedence() == last_op.get_precedence() && op.get_associativity() == Associativity::Left {
                                    reversed_polish.push(op_stack.pop().unwrap().as_expr());
                            }
                            else {break;}
                        }
                    }
                    op_stack.push(Op::Bin(op));
                }
                Expr::Prefix(op) | Expr::Postfixfix(op) => {
                    while op_stack.len() > 0 {
                        if let Some(last_op) = op_stack.last() {
                            if op.get_precedence() < last_op.get_precedence() || 
                                op.get_precedence() == last_op.get_precedence() && op.get_associativity() == Associativity::Left {
                                    reversed_polish.push(op_stack.pop().unwrap().as_expr());
                            }
                            else {break;}
                        }
                    }
                    op_stack.push(Op::Un(op));
                },
            }
        }

        let op_stack = op_stack.iter().map(|op| op.as_expr()).rev();
        reversed_polish.extend(op_stack);

        // println!("We did it! \n {:#?}", reversed_polish);
        Ok(reversed_polish)
    }

    fn compute_expr_vec (val_op_sequence: &Vec<Expr>) -> Result<Val , ParseError> {
        let mut val_op_sequence = Vec::from((*val_op_sequence).clone());

        let find_last_vals = |val_op_sequence: &[Expr], count: u32| -> Vec<usize> {
            let mut last_vals = Vec::new();
            let mut foud = 0;
            let mut counter = val_op_sequence.len()-1;
            for val_op in val_op_sequence.iter().rev(){
                if let Expr::Val(_) = val_op {
                    last_vals.push(counter);
                    foud += 1;
                    if foud == count {break;}
                }
                if counter == 0 {break;}
                counter -= 1;
            }
            last_vals
        };
        let mut i = 0;
        while i < val_op_sequence.len() {
            match val_op_sequence[i] {
                Expr::Val(_) => {i += 1; continue;},
                Expr::Infix(op) => {
                    let operands_positions = find_last_vals(&val_op_sequence[0..i], 2);
                    let rhs = match val_op_sequence.remove(operands_positions[0]){
                        Expr::Val(val) => val,
                        _ => unreachable!(),
                    };
                    let lhs = match val_op_sequence.remove(operands_positions[1]){
                        Expr::Val(val) => val,
                        _ => unreachable!(),
                    };                    
                    i -= 2;
                    let result = op.compute(lhs, rhs).or_else(|e| Err(ParseError{desc: format!("Error in while processing operators: {}", e)}))?;
                    val_op_sequence.remove(i);
                    val_op_sequence.insert(i, Expr::Val(result));
                },
                Expr::Prefix(op) | Expr::Postfixfix(op) => {
                    let operands_positions = find_last_vals(&val_op_sequence[0..i], 1);
                    let lrhs = match val_op_sequence.remove(operands_positions[0]){
                        Expr::Val(val) => val,
                        _ => unreachable!(),
                    };
                    i -= 1;
                    let result = op.compute(lrhs).or_else(|e| Err(ParseError{desc: format!("Error in while processing operators: {}", e)}))?;
                    val_op_sequence.remove(i);
                    val_op_sequence.insert(i, Expr::Val(result));
                },
            }
            i += 1;
        };
        if val_op_sequence.len() == 1 {
            return match &val_op_sequence[0] {
                Expr::Val(val) => Ok(val.clone()),
                _ => unreachable!(),
            };
        }
        unreachable!()
    }

}


#[derive(Parser)]
#[grammar = "expr_parser.pest"]
pub struct MathParser{}

#[cfg(test)]
mod tests{
    use std::cell::RefCell;
    use std::sync::Arc;

    use super::SyntCalc;
    use super::ValOpts;

    #[test]
    fn some_check(){
        let a = SyntCalc::new().eval_str(
            "-1+sin(arcsin(0))+sin(pi)+3*4+5"
        ).unwrap().get_magnetude();
        let b = SyntCalc::new().eval_str(
            "2km*3"
        ).unwrap().get_magnetude();
        assert_eq!(a, 16.);
        assert_eq!(b, 6000.)
    }
}
