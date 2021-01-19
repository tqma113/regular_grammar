# Regular Grammar

A recognizer for strict Regular grammar.

TODO:

+ accept Expanded Regular Grammar

```
regular_expressions -> compound_re* 
compound_re -> repeat_re | simple_re 
repeat_re -> simple_re [’*’|’+’|’?’] 
simple_re -> token | ’(’ regular_expression ’)’
```


```antlrv4
grammar RegularExpression;

regular_expressions: compound_re*;
compound_re: repeat_re | simple_re;
repeat_re: simple_re ['*'|'+'|'?'];
simple_re: token | '(' regular_expression ')';
```
