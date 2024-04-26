mod parse;
use parse::*;
#[cfg(test)]
mod test{
    use crate::parse;

    #[test]
    fn check_function_struct() {
        use super::parse::tokens::Function;
        use super::tokens::Val;
        use super::tokens::val::base_units::*;
        let a = Function::new(|b|{
            return b[0]*Val::new(1./3., D);
        }, 1);
        let v = a.compute(vec![Val::new(2., D)]);
        assert_eq!(v, Ok(Val::new(2./3., D)));
    }
}

