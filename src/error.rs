use std::fmt;

use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub enum Diagnostic {
    Input(InputDiagnostic),
    Rule(RuleDiagnostic),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputDiagnostic {
    c: char,
    index: usize,
    message: String,
}

impl fmt::Display for InputDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl InputDiagnostic {
    pub fn new(c: char, index: usize, message: String) -> Self {
        InputDiagnostic { c, index, message }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuleDiagnostic {
    base: Symbol,
    input: Symbol,
    message: String,
}

impl fmt::Display for RuleDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl RuleDiagnostic {
    pub fn new(base: Symbol, input: Symbol, message: String) -> Self {
        RuleDiagnostic {
            base,
            input,
            message,
        }
    }
}
