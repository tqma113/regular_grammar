use super::symbol::*;

#[macro_export]
macro_rules! regular_grammar {
    (
        Start($start:literal);
        NonTerminals[$($non_terminal:literal),+ $(,)?];
        Terminals[$($terminal:literal),+ $(,)?];
        Rules[$($left:literal => [$([$first:literal,$second:literal]),+ $(,)?]),+ $(,)?];
        TerminalRules[$($t_left:literal => [$($t_right:literal),+ $(,)?]),+ $(,)?]
    ) => {
        {
            let start_terminal = $crate::Symbol::intern($start);

            let mut non_terminals: $crate::HashSet<$crate::Symbol> = $crate::HashSet::new();
            $(
                non_terminals.insert($crate::Symbol::intern($non_terminal));
            )*

            let mut terminals: $crate::HashSet<$crate::Symbol> = $crate::HashSet::new();
            $(
                let symbol = $crate::Symbol::intern($terminal);
                assert!(
                    !non_terminals.contains(&symbol),
                    format!("Non-terminal:{} has already exist in terminal set.", symbol)
                );
                terminals.insert(symbol);
            )*

            let mut rules = $crate::Rules::new();
            $(
                let left = $crate::Symbol::intern($left);
                assert!(
                    non_terminals.contains(&left),
                    format!("The rule's left part: {} is in non-terminals", left)
                );
                let mut right: Vec<$crate::RuleRight> = vec![];
                $(
                    right.push($crate::RuleRight::new(
                        $crate::Symbol::intern($first),
                        $crate::Symbol::intern($second)
                    ));
                )*
                rules.insert(left, right);
            )*

            let mut terminal_rules = $crate::TerminalRules::new();
            $(
                let left = $crate::Symbol::intern($t_left);
                let mut right: Vec<$crate::Symbol> = vec![];
                $(
                    right.push($crate::Symbol::intern($t_right));
                )*
                terminal_rules.insert(left, right);
            )*

            $crate::CNF::new(
                start_terminal,
                non_terminals,
                terminals,
                rules,
                terminal_rules
            )
        }
    };
}

pub trait Grammar {
    fn start_symbol(&self) -> Symbol;

    fn exist(&self, symbol: Symbol) -> bool;

    fn first(&self, symbol: Symbol) -> Option<Vec<Symbol>>;

    fn follow(&self, symbol: Symbol) -> Option<Vec<Symbol>>;

    fn derive(&self, base: Symbol, suffix: Symbol) -> Option<Vec<Symbol>>;

    fn derive_single(&self, base: Symbol) -> Option<Vec<Symbol>>;

    fn is_terminal(&self, input: Symbol) -> bool;

    fn is_non_terminal(&self, input: Symbol) -> bool;
}

#[derive(Debug, Clone)]
pub struct TerminalRule(Symbol, Vec<Symbol>);

impl TerminalRule {
    pub fn start(&self) -> Symbol {
        self.0
    }

    fn derive(&self, base: Symbol) -> Option<Symbol> {
        for symbol in &self.1 {
            if symbol.eq(&base) {
                return Some(self.0);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct TerminalRules(Vec<TerminalRule>);

impl Default for TerminalRules {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalRules {
    pub fn new() -> Self {
        let rules: Vec<TerminalRule> = vec![];
        TerminalRules(rules)
    }

    pub fn insert(&mut self, left: Symbol, right: Vec<Symbol>) {
        self.0.push(TerminalRule(left, right))
    }

    fn derive(&self, base: Symbol) -> Option<Vec<Symbol>> {
        let mut result: Vec<Symbol> = vec![];

        for rule in &self.0 {
            if let Some(symbol) = rule.derive(base) {
                result.push(symbol)
            }
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}