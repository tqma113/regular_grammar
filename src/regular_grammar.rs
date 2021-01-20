use super::symbol::*;

use std::collections::HashMap;
pub use std::collections::HashSet;

#[macro_export]
macro_rules! regular_grammar {
    (
        Start($start:literal);
        Epsilon($epsilon:literal);
        NonTerminals[$($non_terminal:literal),+ $(,)?];
        Terminals[$($terminal:literal),+ $(,)?];
        Rules[$($left:literal => [$([$first:literal,$second:literal]),+ $(,)?]),+ $(,)?];
        TerminalRules[$($t_left:literal => [$($t_right:literal),+ $(,)?]),+ $(,)?]
    ) => {
        {
            let mut non_terminals: $crate::regular_grammar::HashSet<$crate::symbol::Symbol>
                = $crate::regular_grammar::HashSet::new();
            $(
                non_terminals.insert($crate::symbol::Symbol::intern($non_terminal));
            )*

            let start_terminal = $crate::symbol::Symbol::intern($start);
            assert!(
                non_terminals.contains(&start_terminal),
                format!("Start:{} is not exist in non-terminals set", start_terminal)
            );

            let mut terminals: $crate::regular_grammar::HashSet<$crate::symbol::Symbol>
                = $crate::regular_grammar::HashSet::new();
            $(
                let symbol = $crate::symbol::Symbol::intern($terminal);
                assert!(
                    !non_terminals.contains(&symbol),
                    format!("Non-terminal:{} has already exist in terminal set", symbol)
                );

                terminals.insert(symbol);
            )*

            let epsilon_terminal = $crate::symbol::Symbol::intern($epsilon);
            assert!(
                terminals.contains(&epsilon_terminal),
                format!("Epsilon:{} is not exist in terminals set", epsilon_terminal)
            );

            let mut table = $crate::regular_grammar::Table::new();
            $(
                let left = $crate::symbol::Symbol::intern($left);
                assert!(
                    non_terminals.contains(&left),
                    format!("The rule's left part: {} is not exist in non-terminals", left)
                );

                $(
                    let first = $crate::symbol::Symbol::intern($first);
                    assert!(
                        terminals.contains(&first),
                        format!("The rule's first part: {} is not exist in terminal set", first)
                    );

                    let second = $crate::symbol::Symbol::intern($second);
                    assert!(
                        non_terminals.contains(&second),
                        format!("The rule's second part: {} is not exist in non-terminal set", second)
                    );

                    table.insert(left, first, second);
                )*
            )*

            let mut map = $crate::regular_grammar::Map::new();
            $(
                let left = $crate::symbol::Symbol::intern($t_left);
                assert!(
                    non_terminals.contains(&left),
                    format!("The rule's left part: {} is not exist in non-terminal set", left)
                );

                $(
                    let right = $crate::symbol::Symbol::intern($t_right);
                    assert!(
                        terminals.contains(&right),
                        format!("The rule's left part: {} is not exist in terminal set", right)
                    );

                    map.insert(left, right);
                )*
            )*

            $crate::regular_grammar::RegularGrammar::new(
                start_terminal,
                epsilon_terminal,
                non_terminals,
                terminals,
                table,
                map
            )
        }
    };
}

pub enum Next {
    Some(Symbol),
    End,
    None,
}

pub trait Grammar {
    fn next(&self, base: Symbol, input: Symbol) -> Next;

    fn start(&self) -> Symbol;

    fn epsilon(&self) -> Symbol;

    fn exist(&self, symbol: Symbol) -> bool;

    fn is_terminal(&self, symbol: Symbol) -> bool;

    fn is_non_terminal(&self, symbol: Symbol) -> bool;
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct TableKey(Symbol, Symbol);

#[derive(Debug, Clone)]
pub struct Table(HashMap<TableKey, Symbol>);

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Table(HashMap::new())
    }

    pub fn insert(&mut self, base: Symbol, input: Symbol, next: Symbol) -> Option<Symbol> {
        self.0.insert(TableKey(base, input), next)
    }

    fn next(&self, base: Symbol, input: Symbol) -> Option<&Symbol> {
        self.0.get(&TableKey(base, input))
    }
}

#[derive(Debug, Clone)]
pub struct Map(HashMap<Symbol, HashSet<Symbol>>);

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Map(HashMap::new())
    }

    pub fn insert(&mut self, base: Symbol, input: Symbol) {
        match self.0.get(&base) {
            Some(values) => {
                let mut values: HashSet<Symbol> = values.clone();
                values.insert(input);
                self.0.insert(base, values);
            }
            None => {
                let mut values: HashSet<Symbol> = HashSet::new();
                values.insert(input);
                self.0.insert(base, values);
            }
        }
    }

    fn next(&self, base: Symbol, input: Symbol) -> bool {
        if let Some(symbols) = self.0.get(&base) {
            symbols.contains(&input)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    start: Symbol,
    epsilon: Symbol,
    terminals: HashSet<Symbol>,
    non_terminals: HashSet<Symbol>,
    table: Table,
    map: Map,
}

impl RegularGrammar {
    pub fn new(
        start: Symbol,
        epsilon: Symbol,
        terminals: HashSet<Symbol>,
        non_terminals: HashSet<Symbol>,
        table: Table,
        map: Map,
    ) -> Self {
        RegularGrammar {
            start,
            epsilon,
            terminals,
            non_terminals,
            table,
            map,
        }
    }
}

impl Grammar for RegularGrammar {
    fn next(&self, base: Symbol, input: Symbol) -> Next {
        match self.table.next(base, input) {
            Some(symbol) => Next::Some(*symbol),
            None => {
                if self.map.next(base, input) {
                    Next::End
                } else {
                    Next::None
                }
            }
        }
    }

    fn start(&self) -> Symbol {
        self.start
    }

    fn epsilon(&self) -> Symbol {
        self.epsilon
    }

    fn exist(&self, symbol: Symbol) -> bool {
        self.non_terminals.contains(&symbol) || self.terminals.contains(&symbol)
    }

    fn is_terminal(&self, symbol: Symbol) -> bool {
        self.terminals.contains(&symbol)
    }

    fn is_non_terminal(&self, symbol: Symbol) -> bool {
        self.non_terminals.contains(&symbol)
    }
}
