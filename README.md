# SyntCalc
SyncCalc is math expression parser with support for SI unit system, functions, variables and some more.

## Syntax
The syntax is quiet straitforward.
The passed string should either be an expression, a vaiable declaratoion or a function declaratoion.
An expression consists of some amount of `atomics` separated by binary operators or prefixed with unary negation.
These atomics can be 
- numbers(e.g.`123.456E-1`), 
- variables(e.g. `x_1`), 
- functions(e.g. `sin()`), 
- parenthesized expressions (e.g `(1+sin(pi))` ), 
- or ternary operators (e.g `(x > 0 ? 1 : -1)`). 

Numbers can be immediatelly followed by a variable.
E. g.
```
x = 2
2 x \\output: 4
```
All units are essentially variables so they can both be used standalone or after a number.
```
2m //output: 2m
2 * m //output: 2m
m = m^2 //you can even redefine them (may be changed)
```

## Examples
Here are an example of series of valid expressions:
```
x = 2
f(y) = y^2
f(x) //output: 4
g(y) = f(x)^2
g(x) //output: 16
(1+2*3)/3.5 + sin(pi) //output: 2
kg*m^2/s^2 //output: 1J

```

## Improvements to be made 
- Add more units
- Add support for arbitrary precision arithmetic
- Add support for complex numbers (maybe)
- Add recursion depth check
- Make a better shell and/or UI


