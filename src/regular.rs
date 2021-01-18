use super::symbol::*;

pub use std::collections::HashSet;
use std::collections::HashMap;

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
            let mut non_terminals: $crate::HashSet<$crate::Symbol> = $crate::HashSet::new();
            $(
                non_terminals.insert($crate::Symbol::intern($non_terminal));
            )*

            let start_terminal = $crate::Symbol::intern($start);
            assert!(
                non_terminals.contains(&start_terminal),
                format!("Start:{} is not exist in non-terminals set", start_terminal)
            );

            let mut terminals: $crate::HashSet<$crate::Symbol> = $crate::HashSet::new();
            $(
                let symbol = $crate::Symbol::intern($terminal);
                assert!(
                    !non_terminals.contains(&symbol),
                    format!("Non-terminal:{} has already exist in terminal set", symbol)
                );

                terminals.insert(symbol);
            )*

            let epsilon_terminal = $crate::Symbol::intern($epsilon);
            assert!(
                terminals.contains(&epsilon_terminal),
                format!("Epsilon:{} is not exist in terminals set", epsilon_terminal)
            );

            let mut table = $crate::Table::new();
            $(
                let left = $crate::Symbol::intern($left);
                assert!(
                    non_terminals.contains(&left),
                    format!("The rule's left part: {} is not exist in non-terminals", left)
                );

                $(
                    let first = $crate::Symbol::intern($first);
                    assert!(
                        terminals.contains(&first),
                        format!("The rule's first part: {} is not exist in terminal set", first)
                    );

                    let second = $crate::Symbol::intern($second);
                    assert!(
                        non_terminals.contains(&second),
                        format!("The rule's second part: {} is not exist in non-terminal set", second)
                    );

                    table.insert(left, first, second);
                )*
            )*

            let mut map = $crate::Map::new();
            $(
                let left = $crate::Symbol::intern($t_left);
                assert!(
                    non_terminals.contains(&left),
                    format!("The rule's left part: {} is not exist in non-terminal set", left)
                );

                $(
                    let right = $crate::Symbol::intern($t_right);
                    assert!(
                        terminals.contains(&right),
                        format!("The rule's left part: {} is not exist in terminal set", right)
                    );

                    map.insert(left, right);
                )*
            )*

            $crate::Regular::new(
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

pub trait Grammar {
    fn left(&self, first: Symbol, second: Symbol) -> Option<&HashSet<Symbol>>;

    fn first(&self, left: Symbol, second: Symbol) -> Option<&Symbol>;

    fn second(&self, left: Symbol, first: Symbol) -> Option<&Symbol>;

    fn single_left(&self, right: Symbol) -> Option<&HashSet<Symbol>>;

    fn single_right(&self, right: Symbol) -> Option<&HashSet<Symbol>>;

    fn exist(&self, symbol: Symbol) -> bool;

    fn is_terminal(&self, symbol: Symbol) -> bool;

    fn is_non_terminal(&self, symbol: Symbol) -> bool;
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct TableKey(Symbol, Symbol);

#[derive(Debug, Clone)]
pub struct Table {
    lefts: HashMap<TableKey, HashSet<Symbol>>,
    firsts: HashMap<TableKey, Symbol>,
    seconds: HashMap<TableKey, Symbol>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            lefts: HashMap::new(),
            firsts: HashMap::new(),
            seconds: HashMap::new()
        }
    }

    pub fn insert(&mut self, left: Symbol, first: Symbol, second: Symbol) {
        let left_key = TableKey(first, second);
        match self.lefts.get(&left_key) {
            Some(values) => {
                let mut values = values.clone();
                values.insert(left);
                self.lefts.insert(left_key, values);
            }
            None => {
                let mut values: HashSet<Symbol> = HashSet::new();
                values.insert(left);
                self.lefts.insert(left_key, values);
            }
        }
        self.firsts.insert(TableKey(left, second), first);
        self.seconds.insert(TableKey(left, first), second);
    }

    fn left(&self, first: Symbol, second: Symbol) -> Option<&HashSet<Symbol>> {
        self.lefts.get(&TableKey(first, second))
    }

    fn first(&self, left: Symbol, second: Symbol) -> Option<&Symbol> {
        self.firsts.get(&TableKey(left, second))
    }

    fn second(&self, left: Symbol, first: Symbol) -> Option<&Symbol> {
        self.seconds.get(&TableKey(left, first))
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    lefts: HashMap<Symbol, HashSet<Symbol>>,
    rights: HashMap<Symbol, HashSet<Symbol>>
}

impl Map {
    pub fn new() -> Self {
        Map {
            lefts: HashMap::new(),
            rights: HashMap::new()
        }
    }

    pub fn insert(&mut self, left: Symbol, right: Symbol) {
        match self.lefts.get(&right) {
            Some(values) => {
                let mut values: HashSet<Symbol> = values.clone();
                values.insert(left);
                self.lefts.insert(right, values);
            }
            None => {
                let mut values: HashSet<Symbol> = HashSet::new();
                values.insert(left);
                self.lefts.insert(right, values);
            }
        };
        match self.rights.get(&left) {
            Some(values) => {
                let mut values: HashSet<Symbol> = values.clone();
                values.insert(right);
                self.rights.insert(left, values);
            }
            None => {
                let mut values: HashSet<Symbol> = HashSet::new();
                values.insert(right);
                self.rights.insert(left, values);
            }
        };
    }

    fn left(&self, right: Symbol) -> Option<&HashSet<Symbol>> {
        self.lefts.get(&right)
    }

    fn right(&self, left: Symbol) -> Option<&HashSet<Symbol>> {
        self.rights.get(&left)
    }
}

#[derive(Debug, Clone)]
pub struct Regular {
    start: Symbol,
    epsilon: Symbol,
    terminals: HashSet<Symbol>,
    non_terminals: HashSet<Symbol>,
    table: Table,
    map: Map
}

impl Regular {
    pub fn new(
        start: Symbol,
        epsilon: Symbol,
        terminals: HashSet<Symbol>,
        non_terminals: HashSet<Symbol>,
        table: Table,
        map: Map
    ) -> Self {
        Regular {
            start,
            epsilon,
            terminals,
            non_terminals,
            table,
            map
        }
    }
}

impl Grammar for Regular {
    fn left(&self, first: Symbol, second: Symbol) -> Option<&HashSet<Symbol>> {
        self.table.left(first, second)
    }

    fn first(&self, left: Symbol, second: Symbol) -> Option<&Symbol> {
        self.table.first(left, second)
    }

    fn second(&self, left: Symbol, first: Symbol) -> Option<&Symbol> {
        self.table.second(left, first)
    }

    fn single_left(&self, right: Symbol) -> Option<&HashSet<Symbol>> {
        self.map.left(right)
    }

    fn single_right(&self, left: Symbol) -> Option<&HashSet<Symbol>> {
        self.map.right(left)
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