mod parse;
use parse::*;
#[cfg(test)]
mod test{
    use crate::tokens::val::{Unit, units::ValAlias};
    use crate::{parse, tokens::Val};
    use super::tokens::val::base_units::*;
    use regex::Regex;

    #[test]
    fn check_function_token_compute() {
        use super::parse::tokens::Function;
        let a = Function::new(|b|{
            return b[0].clone()*Val::new(1./3., D);
        }, 1);
        let v = a.compute(vec![Val::new(2., D)]);
        assert_eq!(v, Ok(Val::new(2./3., D)));
    }
    
    #[test]
    fn check_val_comparsion() {
        let a = Val::new(1., D);
        let b = Val::new(1., D);
        let c = Val::new(1., KG);
        let d = Val::new(1., Unit::new([0.,0.001, 0., 0., 0., 0., 0.]));
        assert_eq!(a.same_unit(&b, None), true);
        assert_eq!(a.same_unit(&c, None), false);
        assert_eq!(a.same_unit(&d, None), true);
        assert_eq!(a.same_unit(&d, Some(0.0001)), false);
    }

    #[test]
    fn check_val_math(){
        let al = ValAlias::new();
        let a = al.get_val("W").unwrap();
        let b = al.get_val("J").unwrap();
        let c = Val::new(3., al.get_val("W").unwrap().get_unit());
        assert_eq!((b.clone()/a.clone()) == al.get_val("s").unwrap(), true);
        assert_eq!((c/a) == Val::new(3., D), true);
    }
    

    #[test]
    fn tokens_from_str(){
        let expr = "-123.12E0";
        let res = -123.12;
        let a = Val::from_str(expr).unwrap().get_magnetude();
        assert_eq!(a, res);
    }
}

