use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::multi::many0;
use nom::IResult;
use std::convert::TryFrom;

static BRAINFUCK_BYTES: &[u8] = b"><+-.,[]";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenTree(pub Vec<Token>);

impl TokenTree {
    pub fn from_str(program: &str) -> Self {
        let program = program
            .bytes()
            .filter(|byte| BRAINFUCK_BYTES.contains(byte))
            .collect::<Vec<u8>>();
        let (_, tokens) =
            many0(alt((TokenTree::action, TokenTree::action_loop)))(&program).unwrap();
        TokenTree(tokens)
    }

    fn action(i: &[u8]) -> IResult<&[u8], Token> {
        let (i, char) = one_of("><+-.,")(i)?;
        Ok((i, Token::try_from(char).unwrap()))
    }

    fn action_loop(i: &[u8]) -> IResult<&[u8], Token> {
        let (i, _) = char('[')(i)?;
        let (i, tokens) = many0(alt((TokenTree::action, TokenTree::action_loop)))(i)?;
        let (i, _) = char(']')(i)?;
        Ok((i, Token::Loop(tokens)))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    ReadByte,
    WriteByte,
    Loop(Vec<Token>),
}

impl TryFrom<char> for Token {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Token::MoveRight),
            '<' => Ok(Token::MoveLeft),
            '+' => Ok(Token::Increment),
            '-' => Ok(Token::Decrement),
            '.' => Ok(Token::WriteByte),
            ',' => Ok(Token::ReadByte),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenTree};

    #[test]
    fn action_token() {
        let expected_pairs = vec![
            (Token::MoveRight, b'>'),
            (Token::MoveLeft, b'<'),
            (Token::Increment, b'+'),
            (Token::Decrement, b'-'),
            (Token::WriteByte, b'.'),
            (Token::ReadByte, b','),
        ];
        for expected_pair in expected_pairs {
            assert_eq!(
                expected_pair.0,
                TokenTree::action(&[expected_pair.1]).unwrap().1
            );
        }
    }

    #[test]
    fn empty_loop() {
        assert_eq!(
            Token::Loop(vec![]),
            TokenTree::action_loop(b"[]").unwrap().1
        );
    }

    #[test]
    fn nested_empty_loop() {
        assert_eq!(
            Token::Loop(vec![Token::Loop(vec![])]),
            TokenTree::action_loop(b"[[]]").unwrap().1
        );
    }

    #[test]
    fn non_empty_loop() {
        assert_eq!(
            Token::Loop(vec![
                Token::Increment,
                Token::WriteByte,
                Token::MoveRight,
                Token::ReadByte,
                Token::Decrement,
                Token::MoveLeft
            ]),
            TokenTree::action_loop(b"[+.>,-<]").unwrap().1
        );
    }

    #[test]
    fn actions_with_loop() {
        assert_eq!(
            TokenTree(vec![
                Token::Increment,
                Token::Loop(vec![Token::Decrement]),
                Token::WriteByte
            ]),
            TokenTree::from_str("+[-].")
        );
    }

    #[test]
    fn actions_with_nested_loop() {
        assert_eq!(
            TokenTree(vec![
                Token::Increment,
                Token::Loop(vec![
                    Token::Decrement,
                    Token::Loop(vec![
                        Token::MoveRight,
                        Token::Increment,
                        Token::Loop(vec![]),
                        Token::Increment,
                        Token::Loop(vec![]),
                    ]),
                    Token::ReadByte
                ]),
                Token::WriteByte
            ]),
            TokenTree::from_str("+[-[>+[]+[]],].")
        );
    }

    #[test]
    fn adjacent_loops() {
        assert_eq!(
            TokenTree(vec![
                Token::Loop(vec![]),
                Token::Loop(vec![]),
                Token::Loop(vec![]),
            ]),
            TokenTree::from_str("[][][]")
        );
    }

    #[test]
    fn actions_with_adjacent_loops() {
        assert_eq!(
            TokenTree(vec![
                Token::Increment,
                Token::Loop(vec![]),
                Token::Loop(vec![]),
                Token::Loop(vec![]),
                Token::Increment,
            ]),
            TokenTree::from_str("+[][][]+")
        );
        assert_eq!(
            TokenTree(vec![
                Token::Increment,
                Token::Loop(vec![]),
                Token::Increment,
                Token::Loop(vec![]),
                Token::Increment,
                Token::Loop(vec![]),
                Token::Increment,
            ]),
            TokenTree::from_str("+[]+[]+[]+")
        );
    }

    #[test]
    fn actions_with_adjacent_nested_loops() {
        assert_eq!(
            TokenTree(vec![
                Token::Increment,
                Token::Loop(vec![
                    Token::Decrement,
                    Token::Loop(vec![Token::Increment]),
                    Token::Decrement
                ]),
                Token::Increment,
                Token::Loop(vec![Token::Decrement]),
                Token::Increment
            ]),
            TokenTree::from_str("+[-[+]-]+[-]+")
        );
    }
}
