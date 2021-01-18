# Regular Expression

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

```
R -> 
```