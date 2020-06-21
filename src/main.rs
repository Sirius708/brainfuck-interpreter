use brainfuck_interpreter::run_program;
use std::env::args;

fn main() {
    let program = args().nth(1).unwrap();
    run_program(&program);
}
