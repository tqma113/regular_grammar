use std::fmt;

use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub enum Diagnostic {
    Input(InputDiagnostic),
    Rule(RuleDiagnostic),
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(input) => fmt::Display::fmt(&input, f),
            Self::Rule(rule) => fmt::Display::fmt(&rule, f)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputDiagnostic {
    c: char,
    index: usize,
}

impl fmt::Display for InputDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown input:{} at {}", self.c, self.index)
    }
}

impl InputDiagnostic {
    pub fn new(c: char, index: usize) -> Self {
        InputDiagnostic { c, index }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuleDiagnostic {
    base: Symbol,
    input: Symbol,
}

impl fmt::Display for RuleDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Non-Productive Rules. Base: {}, input: {}", self.base, self.input)
    }
}

impl RuleDiagnostic {
    pub fn new(base: Symbol, input: Symbol) -> Self {
        RuleDiagnostic {
            base,
            input,
        }
    }
}
