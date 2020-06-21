use crate::tape::Tape;
use crate::token::{Token, TokenTree};

mod tape;
mod token;

fn process_token(tape: &mut Tape, token: &Token) {
    match token {
        Token::MoveRight => tape.move_pointer_right(),
        Token::MoveLeft => tape.move_pointer_left(),
        Token::Increment => tape.increment_cell(),
        Token::Decrement => tape.decrement_cell(),
        Token::ReadByte => tape.read_input(),
        Token::WriteByte => tape.print_cell(),
        Token::Loop(loop_tokens) => {
            while !tape.is_cell_zero() {
                for token in loop_tokens {
                    process_token(tape, token);
                }
            }
        }
    }
}

pub fn run_program(program: &str) {
    let program = TokenTree::from_str(&program);
    let mut tape = Tape::new();
    for token in program.0 {
        process_token(&mut tape, &token);
    }
}
