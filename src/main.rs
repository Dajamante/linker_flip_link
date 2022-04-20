//! This lexer is intended to be used with Ferrous Systems flip-link

use std::iter::{Enumerate, Peekable};
use std::str::Chars;

fn main() {}

#[derive(Debug, PartialEq)]
struct Token {
    line_number: usize,
    token_type: TokenType,
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
    let mut vec_tokens = Vec::new();

    let mut line_number = 0;

    //let mut it = script.chars().peekable();
    let mut it = script.chars().enumerate().peekable();
    while let Some((ind, ch)) = it.next() {
        println!("this is ch: {}", ch);
        match ch {
            ':' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::Colon,
            }),
            ',' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::Comma,
            }),
            '{' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::CurlyOpen,
            }),
            '}' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::CurlyClose,
            }),
            '=' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::Equal,
            }),
            '(' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::ParOpen,
            }),
            ')' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::ParClose,
            }),
            '.' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::Dot,
            }),
            ' ' | '\t' | '\r' => {}
            '\n' => {
                line_number += 1;
            }
            '/' => {
                // Multiblock comment
                if let Some((_, '*')) = it.peek() {
                    let _next_star = advance_while(&mut it, |c| *c == '*');
                    let stop = advance_while(&mut it, |c| *c == '/').unwrap_or(script.len());
                    // A multiblock comment will have several new lines
                    // Not the beautifulest but we can add it as an optional argument
                    // in the advance_while() function
                    line_number += script[ind..stop].chars().filter(|c| *c == '\n').count();
                }
                // line comment
                if let Some((_, '/')) = it.peek() {
                    // must iterate until /n
                    let _ = advance_while(&mut it, |ch| *ch == '\n');
                }
            }
            '+' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::Plus,
            }),
            // Assuming that hex number always start with a 0, and not an "x"!
            '0'..='9' => {
                let mut start = ind;
                let radix = if let Some((_, 'x')) = it.peek() {
                    // Consumme the "x"
                    it.next();
                    // the base 16 number starts after "0x"
                    start += 2;
                    16
                } else {
                    10
                };
                let stop = advance_while(it.by_ref(), |c: &char| !c.is_ascii_hexdigit())
                    .unwrap_or(script.len());

                let number = u64::from_str_radix(&script[start..stop], radix).unwrap();

                vec_tokens.push(Token {
                    line_number,
                    token_type: TokenType::Number(number),
                });
            }
            'a'..='z' | 'A'..='Z' => {
                let start = ind;

                let stop = advance_while(it.by_ref(), |c: &char| !c.is_alphabetic())
                    .unwrap_or(script.len());

                vec_tokens.push(Token {
                    line_number,
                    token_type: TokenType::Word(script[start..stop].to_string()),
                })
            }
            _ => {
                continue;
            }
        }
    }

    vec_tokens
}

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
        assert_eq!(lexer("0"), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lexer_2_1_with_space_before() {
        assert_eq!(lexer(" 0"), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lexer_2_1_with_space_after() {
        assert_eq!(lexer("0 "), vec![create_token_number(0, 0)]);
    }
    #[test]
    fn lexer_2_1_with_space_after_12() {
        assert_eq!(lexer("12 "), vec![create_token_number(0, 12)]);
    }

    #[test]
    fn lexer_2_1_with_space_before_after() {
        assert_eq!(lexer(" 0 "), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lexer_2_1_with_tabs() {
        assert_eq!(lexer("  \t  0     "), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lexer_2_2() {
        assert_eq!(lexer("1"), vec![create_token_number(0, 1)]);
    }

    #[test]
    fn lexer_2_3() {
        assert_eq!(lexer("12"), vec![create_token_number(0, 12)]);
    }

    #[test]
    fn lexer_2_4() {
        assert_eq!(lexer("00000001"), vec![create_token_number(0, 1)]);
    }

    #[test]
    fn lexer_2_5() {
        assert_eq!(lexer("0x0"), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lexer_2_6() {
        assert_eq!(lexer("0x1"), vec![create_token_number(0, 1)]);
    }

    #[test]
    fn lexer_2_7() {
        assert_eq!(lexer("0x0017"), vec![create_token_number(0, 0x0017)]);
    }

    #[test]
    fn lexer_2_8() {
        assert_eq!(lexer("0x0Af17"), vec![create_token_number(0, 0xaf17)]);
    }
    #[test]
    fn lexer_3() {
        let three_elems: Vec<Token> = vec![
            create_token_word(0, "nx"),
            create_token_word(0, "nx"),
            create_token_number(0, 1),
        ];

        assert_eq!(lexer("nx nx 0x1"), three_elems);
    }

    #[test]
    fn lexer_4_1() {
        let test_new_lines = "
//
0";
        assert_eq!(lexer(test_new_lines), vec![create_token_number(2, 0)]);
    }

    #[test]
    fn lexer_4_2() {
        let test_new_lines = "
// some comment
0";
        assert_eq!(lexer(test_new_lines), vec![create_token_number(2, 0)]);
    }

    #[test]
    fn lexer_4_3() {
        let test_new_lines = "
// some comment
0
// some comment
// more comment";
        assert_eq!(lexer(test_new_lines), vec![create_token_number(2, 0)]);
    }

    #[test]
    fn lexer_4_4() {
        let test_new_lines = "
/*
Very unclear comment
*/
0";
        assert_eq!(lexer(test_new_lines), vec![create_token_number(4, 0)]);
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
            vec![create_token_number(0, 256), create_token_word(0, "K")]
        );
    }

    #[test]
    fn lexer_6_1() {
        let number_and_unit = "256, 368";
        assert_eq!(
            lexer(number_and_unit),
            vec![
                create_token_number(0, 256),
                Token {
                    line_number: 0,
                    token_type: TokenType::Comma
                },
                create_token_number(0, 368)
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
                create_token_number(1, 256),
                Token {
                    line_number: 2,
                    token_type: TokenType::Comma
                },
                create_token_number(3, 368)
            ]
        );
    }

    #[test]
    fn lexer_7_1_1() {
        const LINKER_SCRIPT: &str = "MEMORY
        PROBLEMS
";

        let expected = vec![
            create_token_word(0, "MEMORY"),
            create_token_word(1, "PROBLEMS"),
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
            create_token_word(0, "MEMORY"),
            Token {
                line_number: 1,
                token_type: TokenType::CurlyOpen,
            },
            create_token_word(2, "FLASH"),
            Token {
                line_number: 2,
                token_type: TokenType::Colon,
            },
            create_token_word(2, "ORIGIN"),
            Token {
                line_number: 2,
                token_type: TokenType::Equal,
            },
            create_token_number(2, 0),
            Token {
                line_number: 2,
                token_type: TokenType::Comma,
            },
            create_token_word(2, "LENGTH"),
            Token {
                line_number: 2,
                token_type: TokenType::Equal,
            },
            create_token_number(2, 256),
            create_token_word(2, "K"),
            Token {
                line_number: 3,
                token_type: TokenType::CurlyClose,
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
            create_token_word(0, "MEMORY"),
            create_token_word(1, "LINKER"),
            Token {
                line_number: 1,
                token_type: TokenType::Dot,
            },
            create_token_word(1, "x"),
        ];

        assert_eq!(lexer(LINKER_SCRIPT), expected);
    }

    fn create_token_number(line: usize, number: u64) -> Token {
        Token {
            line_number: line,
            token_type: TokenType::Number(number),
        }
    }

    fn create_token_word(line: usize, string: &str) -> Token {
        Token {
            line_number: line,
            token_type: TokenType::Word(string.to_string()),
        }
    }
}
