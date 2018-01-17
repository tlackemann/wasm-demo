#[macro_use] extern crate stdweb;
extern crate serde_json;
extern crate ttml;

use ttml::parser::parse_step_p;

fn say_hello(name: String) -> String {
    "Hello, ".to_string() + &name
}

fn run_program(input: String) -> String {
    let (_, result) = parse_step_p(b"!r 1d20").unwrap();
    serde_json::to_string(&result).unwrap()
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.hello = @{say_hello};
        Module.exports.run_program = @{run_program};
    }
}
