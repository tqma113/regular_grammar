extern crate regexp;

use regexp::{regular_grammar};

fn main() {
    /// Detail in 5.3 and 5.4 in Parsing Technique.
    /// S -> (ab)*(q|p)+
    /// S -> A
    /// A -> aB
    /// B -> bA
    /// A -> pC
    /// A -> qC
    /// C -> qC
    /// C -> pC
    /// C -> ε
    let grammar = regular_grammar!(
        Start("A");
        Epsilon("ε");
        NonTerminals [
            "A", "B", "C"
        ];
        Terminals [
            "a", "b", "p", "q", "ε"
        ];
        Rules [
            "A" => [
                ["a", "B"],
                ["p", "C"],
                ["q", "C"],
            ],
            "B" => [
                ["b", "A"]
            ],
            "C" => [
                ["p", "C"],
                ["q", "C"],
            ]
        ];
        TerminalRules [
            "C" => ["ε"]
        ]
    );
    println!("{:?}", grammar);
    let mut recognizer = regexp::Recognizer::new(&grammar);
    match recognizer.recognize("abababq") {
        Ok(_) => {
            println!("Ok");
        }
        Err(unknown) => println!("Err: {:?}", unknown),
    }
}
