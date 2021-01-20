# Regular Grammar

Detail in 5.3 in Parsing Technique.

A recognizer for strict Regular grammar.

LL

With Finite state automata.

Time requirements: `O(n)`
Memory requirements: `O(n)`

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
