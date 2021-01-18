# Regular Expression

```
regular_expressions -> compound_re* 
compound_re -> repeat_re | simple_re 
repeat_re -> simple_re [’*’|’+’|’?’] 
simple_re -> token | ’(’ regular_expression ’)’
```
