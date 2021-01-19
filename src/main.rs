extern crate regexp;

use regexp::{regular_grammar};

fn main() {
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
