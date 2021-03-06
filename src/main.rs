//! This lexer is intended to be used with Ferrous Systems flip-link

use std::iter::{Enumerate, Peekable};
use std::str::Chars;

fn main() {}

// TODO: starting and stopping positions
#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    from: usize,
    to: usize,
    line_number: usize,
}
#[derive(Debug, PartialEq)]
enum TokenType {
    Plus,
    Colon,
    CurlyClose,
    CurlyOpen,
    Equal,
    Number(u64),
    Word(String),
    Comma,
    Dot,
    ParClose,
    ParOpen,
}

fn lexer(script: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // atm flip-link is using a line number to write
    // back to the original linker script.
    let mut line_number = 0;

    let mut it = script.chars().enumerate().peekable();
    while let Some((index, ch)) = it.next() {
        let mut push_token = |token_type, from, to| {
            tokens.push(Token {
                token_type,
                from,
                to,
                line_number,
            })
        };
        let mut push_char = |token_type| push_token(token_type, index, index + 1);

        match ch {
            ':' => push_char(TokenType::Colon),
            ',' => push_char(TokenType::Comma),
            '{' => push_char(TokenType::CurlyOpen),
            '}' => push_char(TokenType::CurlyClose),
            '=' => push_char(TokenType::Equal),
            '(' => push_char(TokenType::ParOpen),
            ')' => push_char(TokenType::ParClose),
            '.' => push_char(TokenType::Dot),
            ' ' | '\t' | '\r' => {
                // nothing to do with whitespaces atm
                // if we implement start and stop positions
                // we will need to increment  in that block
            }
            '\n' => {
                line_number += 1;
            }
            '/' => {
                // Multiblock comment
                if let Some((_, '*')) = it.peek() {
                    // eating the '*' character
                    let _ = advance_while(&mut it, |c| *c == '*');
                    let stop = advance_while(&mut it, |c| *c == '/').unwrap_or(script.len());
                    // Findex all new lines in the multiline comment
                    line_number += script[index..stop].chars().filter(|c| *c == '\n').count();
                }
                // One line comment
                if let Some((_, '/')) = it.peek() {
                    // must iterate until /n (end of line)
                    let _ = advance_while(&mut it, |ch| *ch == '\n');
                }
            }
            '+' => push_char(TokenType::Plus),
            // Assuming that hex number always start with a 0, and not an "x"!
            '0'..='9' => {
                let mut from = index;
                let radix = if let Some((_, 'x')) = it.peek() {
                    // Consumme the "x"
                    it.next();
                    from += 2;
                    16
                } else {
                    10
                };
                let to = advance_while(it.by_ref(), |c: &char| !c.is_ascii_hexdigit())
                    .unwrap_or(script.len());

                // Tighten up error management at this stage, or by the parser? What about negative numbers etc.
                let number = u64::from_str_radix(&script[from..to], radix).unwrap();

                push_token(TokenType::Number(number), index, to);
            }
            'a'..='z' | 'A'..='Z' => {
                let to = advance_while(it.by_ref(), |c: &char| !c.is_alphabetic())
                    .unwrap_or(script.len());

                push_token(TokenType::Word(script[index..to].to_string()), index, to)
            }
            // to be decided: substraction? division? multiplication ..?
            _ => {
                continue;
            }
        }
    }

    tokens
}

/// This function advances in the script until it hits a
/// predicate, and returns the index.
fn advance_while(
    it: &mut Peekable<Enumerate<Chars<'_>>>,
    pred: fn(&char) -> bool,
) -> Option<usize> {
    let stop = loop {
        let &(idx, ch) = match it.peek() {
            Some(it) => it,
            None => return None,
        };
        if pred(&ch) {
            break idx;
        }
        it.next();
    };
    Some(stop)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexer_1() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lexer(""), empty);
    }

    #[test]
    fn lexer_1_1() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lexer(" "), empty);
    }

    #[test]
    fn lexer_1_2() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lexer("     "), empty);
    }

    #[test]
    fn lexer_1_2_with_tab() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lexer("    \t "), empty);
    }
    #[test]
    fn lexer_2_1() {
        assert_eq!(lexer("0"), vec![create_token_number(0, 0, 1, 0)]);
    }

    #[test]
    fn lexer_2_1_with_space_before() {
        assert_eq!(lexer(" 0"), vec![create_token_number(0, 1, 2, 0)]);
    }

    #[test]
    fn lexer_2_1_with_space_after() {
        assert_eq!(lexer("0 "), vec![create_token_number(0, 0, 1, 0)]);
    }
    #[test]
    fn lexer_2_1_with_space_after_12() {
        assert_eq!(lexer("12 "), vec![create_token_number(12, 0, 2, 0)]);
    }

    #[test]
    fn lexer_2_1_with_space_before_after() {
        assert_eq!(lexer(" 0 "), vec![create_token_number(0, 1, 2, 0)]);
    }

    #[test]
    fn lexer_2_1_with_tabs() {
        assert_eq!(lexer("  \t  0     "), vec![create_token_number(0, 5, 6, 0)]);
    }

    #[test]
    fn lexer_2_2() {
        assert_eq!(lexer("1"), vec![create_token_number(1, 0, 1, 0)]);
    }

    #[test]
    fn lexer_2_3() {
        assert_eq!(lexer("12"), vec![create_token_number(12, 0, 2, 0)]);
    }

    #[test]
    fn lexer_2_4() {
        assert_eq!(lexer("00000001"), vec![create_token_number(1, 0, 8, 0)]);
    }

    #[test]
    fn lexer_2_5() {
        assert_eq!(lexer("0x0"), vec![create_token_number(0, 0, 3, 0)]);
    }

    #[test]
    fn lexer_2_6() {
        assert_eq!(lexer("0x1"), vec![create_token_number(1, 0, 3, 0)]);
    }

    #[test]
    fn lexer_2_7() {
        assert_eq!(lexer("0x0017"), vec![create_token_number(0x0017, 0, 6, 0)]);
    }

    #[test]
    fn lexer_2_8() {
        assert_eq!(lexer("0x0Af17"), vec![create_token_number(0xaf17, 0, 7, 0)]);
    }
    #[test]
    fn lexer_3() {
        let three_elems: Vec<Token> = vec![
            create_token_word("nx", 0, 2, 0),
            create_token_word("nx", 3, 5, 0),
            create_token_number(1, 6, 9, 0),
        ];

        assert_eq!(lexer("nx nx 0x1"), three_elems);
    }

    #[test]
    fn lexer_4_1() {
        let test_new_lines = "
//
0";
        assert_eq!(lexer(test_new_lines), vec![create_token_number(0, 4, 5, 2)]);
    }

    #[test]
    fn lexer_4_2() {
        let test_new_lines = "
// some comment
0";
        assert_eq!(
            lexer(test_new_lines),
            vec![create_token_number(0, 17, 18, 2)]
        );
    }

    #[test]
    fn lexer_4_3() {
        let test_new_lines = "
// some comment
0
// some comment
// more comment";
        assert_eq!(
            lexer(test_new_lines),
            vec![create_token_number(0, 17, 18, 2)]
        );
    }

    #[test]
    fn lexer_4_4() {
        let test_new_lines = "
/*
Very unclear comment
*/
0";
        assert_eq!(
            lexer(test_new_lines),
            vec![create_token_number(0, 28, 29, 4)]
        );
    }

    #[test]
    fn lexer_4_5() {
        assert_eq!(lexer("/**/"), Vec::new());
    }
    #[test]
    fn lexer_4_6() {
        assert_eq!(lexer("//"), Vec::new());
    }

    #[test]
    fn lexer_5() {
        let number_and_unit = "256K";
        assert_eq!(
            lexer(number_and_unit),
            vec![
                create_token_number(256, 0, 3, 0),
                create_token_word("K", 3, 4, 0)
            ]
        );
    }

    #[test]
    fn lexer_6_1() {
        let number_and_unit = "256, 368";
        assert_eq!(
            lexer(number_and_unit),
            vec![
                create_token_number(256, 0, 3, 0),
                Token {
                    token_type: TokenType::Comma,
                    from: 3,
                    to: 4,
                    line_number: 0,
                },
                create_token_number(368, 5, 8, 0)
            ]
        );
    }

    #[test]
    fn lexer_6_2() {
        let number_and_unit = "
256
,
368";
        assert_eq!(
            lexer(number_and_unit),
            vec![
                create_token_number(256, 1, 4, 1),
                Token {
                    token_type: TokenType::Comma,
                    from: 5,
                    to: 6,
                    line_number: 2,
                },
                create_token_number(368, 7, 10, 3)
            ]
        );
    }

    #[test]
    fn lexer_7_1_1() {
        const LINKER_SCRIPT: &str = "MEMORY
PROBLEMS
";

        let expected = vec![
            create_token_word("MEMORY", 0, 6, 0),
            create_token_word("PROBLEMS", 7, 15, 1),
        ];
        assert_eq!(lexer(LINKER_SCRIPT), expected);
    }
    #[test]
    fn lexer_7_1() {
        const LINKER_SCRIPT: &str = "MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
}
";
        let expected = vec![
            create_token_word("MEMORY", 0, 6, 0),
            Token {
                token_type: TokenType::CurlyOpen,
                from: 7,
                to: 8,
                line_number: 1,
            },
            create_token_word("FLASH", 13, 18, 2),
            Token {
                token_type: TokenType::Colon,
                from: 19,
                to: 20,
                line_number: 2,
            },
            create_token_word("ORIGIN", 21, 27, 2),
            Token {
                token_type: TokenType::Equal,
                from: 28,
                to: 29,
                line_number: 2,
            },
            create_token_number(0, 30, 40, 2),
            Token {
                token_type: TokenType::Comma,
                from: 40,
                to: 41,
                line_number: 2,
            },
            create_token_word("LENGTH", 42, 48, 2),
            Token {
                token_type: TokenType::Equal,
                from: 49,
                to: 50,
                line_number: 2,
            },
            create_token_number(256, 51, 54, 2),
            create_token_word("K", 54, 55, 2),
            Token {
                token_type: TokenType::CurlyClose,
                from: 56,
                to: 57,
                line_number: 3,
            },
        ];

        assert_eq!(lexer(LINKER_SCRIPT), expected);
    }

    #[test]
    fn lexer_7_2() {
        const LINKER_SCRIPT: &str = "MEMORY
LINKER.x
";

        let expected = vec![
            create_token_word("MEMORY", 0, 6, 0),
            create_token_word("LINKER", 7, 13, 1),
            Token {
                token_type: TokenType::Dot,
                from: 13,
                to: 14,
                line_number: 1,
            },
            create_token_word("x", 14, 15, 1),
        ];

        assert_eq!(lexer(LINKER_SCRIPT), expected);
    }

    fn create_token_number(number: u64, from: usize, to: usize, line: usize) -> Token {
        Token {
            token_type: TokenType::Number(number),
            from,
            to,
            line_number: line,
        }
    }
    fn create_token_word(string: &str, from: usize, to: usize, line: usize) -> Token {
        Token {
            token_type: TokenType::Word(string.to_string()),
            from,
            to,
            line_number: line,
        }
    }
}
