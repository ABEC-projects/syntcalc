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

## Examples
Here are an example of series of valid expressions:
```
x = 2
f(y) = y^2
f(x) //output: 4
g(y) = f(x)^2
g(x) //output: 16
(1+2*3)/3.5 + sin(pi) //output: 2
```

## Improvements to be made 
- Add support for outputting number's units (it can only read and process them for now)
- Add support for arbitrary precision arithmetic
- Add support for recursion
- Add support for complex numbers (maybe)


