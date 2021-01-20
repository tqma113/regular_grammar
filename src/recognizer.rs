use super::error::{Diagnostic, InputDiagnostic, RuleDiagnostic};
use super::regular_grammar::{Grammar, Next};
use super::symbol::Symbol;

use std::fmt::Debug;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Recognizer<'a, G: Grammar + Debug + Clone> {
    grammar: &'a G,
    src: &'a str,
    chars: Chars<'a>,

    index: usize,
    state: Symbol,
}

pub enum NextState {
    Some(Symbol),
    Err(Diagnostic),
    End,
}

impl<'a, G: Grammar + Debug + Clone> Recognizer<'a, G> {
    pub fn new(grammar: &'a G) -> Self {
        Recognizer {
            grammar,
            src: "",
            chars: "".chars(),
            index: 0,
            state: grammar.start(),
        }
    }

    pub fn recognize(&mut self, string: &'a str) -> Result<(), Diagnostic> {
        self.src = string;
        self.chars = string.chars();

        loop {
            match self.next() {
                NextState::Some(symbol) => {
                    self.state = symbol;
                    self.index += 1;
                }
                NextState::End => return Ok(()),
                NextState::Err(diagnostic) => return Err(diagnostic),
            }
        }
    }

    fn next(&mut self) -> NextState {
        match self.chars.next() {
            Some(c) => match Symbol::from_char(c) {
                Some(input) => self.next_state(input),
                None => NextState::Err(Diagnostic::Input(InputDiagnostic::new(
                    c,
                    self.index,
                ))),
            },
            None => self.next_state(self.grammar.epsilon()),
        }
    }

    fn next_state(&self, input: Symbol) -> NextState {
        match self.grammar.next(self.state, input) {
            Next::Some(symbol) => NextState::Some(symbol),
            Next::End => NextState::End,
            Next::None => NextState::Err(Diagnostic::Rule(RuleDiagnostic::new(
                self.state,
                input,
            ))),
        }
    }
}
