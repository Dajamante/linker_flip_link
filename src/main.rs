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
fn lex(script: &str) -> Vec<Token> {
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
                println!("This is a new line: {}", ch);
                line_number += 1;
            }
            '/' => {
                // block comment
                if let Some((_, '*')) = it.peek() {
                    let stop = script[ind..].find("*/").unwrap_or(script.len());
                    for b in script[ind..stop].chars() {
                        if b == '\n' {
                            line_number += 1;
                        }
                        it.next();
                    }
                    // after stop there should be a new line
                    line_number += 1;
                }

                // line comment
                if let Some((_, '/')) = it.peek() {
                    // must iterate until /n and increment line_number
                    it.by_ref().find(|(_, ch)| *ch == '\n');
                    line_number += 1;
                }
            }
            '+' => vec_tokens.push(Token {
                line_number,
                token_type: TokenType::Plus,
            }),
            // Assuming that hex number always start with a 0, and not an "x".
            '0'..='9' => {
                let mut start = ind;
                let radix = if let Some((_, 'x')) = it.peek() {
                    // Skip the "x"
                    it.next();
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

                let stop = loop {
                    let &(ind, ch) = match it.peek() {
                        Some(it) => it,
                        None => break script.len(),
                    };

                    if !ch.is_alphabetic() {
                        break ind;
                    }
                    it.next();
                };

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
    fn lex_1() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lex(""), empty);
    }

    #[test]
    fn lex_1_1() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lex(" "), empty);
    }

    #[test]
    fn lex_1_2() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lex("     "), empty);
    }

    #[test]
    fn lex_1_2_with_tab() {
        let empty: Vec<Token> = Vec::new();
        assert_eq!(lex("    \t "), empty);
    }
    #[test]
    fn lex_2_1() {
        assert_eq!(lex("0"), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lex_2_1_with_space_before() {
        assert_eq!(lex(" 0"), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lex_2_1_with_space_after() {
        assert_eq!(lex("0 "), vec![create_token_number(0, 0)]);
    }
    #[test]
    fn lex_2_1_with_space_after_12() {
        assert_eq!(lex("12 "), vec![create_token_number(0, 12)]);
    }

    #[test]
    fn lex_2_1_with_space_before_after() {
        assert_eq!(lex(" 0 "), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lex_2_1_with_tabs() {
        assert_eq!(lex("  \t  0     "), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lex_2_2() {
        assert_eq!(lex("1"), vec![create_token_number(0, 1)]);
    }

    #[test]
    fn lex_2_3() {
        assert_eq!(lex("12"), vec![create_token_number(0, 12)]);
    }

    #[test]
    fn lex_2_4() {
        assert_eq!(lex("00000001"), vec![create_token_number(0, 1)]);
    }

    #[test]
    fn lex_2_5() {
        assert_eq!(lex("0x0"), vec![create_token_number(0, 0)]);
    }

    #[test]
    fn lex_2_6() {
        assert_eq!(lex("0x1"), vec![create_token_number(0, 1)]);
    }

    #[test]
    fn lex_2_7() {
        assert_eq!(lex("0x0017"), vec![create_token_number(0, 0x0017)]);
    }

    #[test]
    fn lex_2_8() {
        assert_eq!(lex("0x0Af17"), vec![create_token_number(0, 0xaf17)]);
    }
    #[test]
    fn lex_3() {
        let three_elems: Vec<Token> = vec![
            create_token_word(0, "nx"),
            create_token_word(0, "nx"),
            create_token_number(0, 1),
        ];

        assert_eq!(lex("nx nx 0x1"), three_elems);
    }

    #[test]
    fn lex_4_1() {
        let test_new_lines = "
//
0";
        assert_eq!(lex(test_new_lines), vec![create_token_number(2, 0)]);
    }

    #[test]
    fn lex_4_2() {
        let test_new_lines = "
// some comment
0";
        assert_eq!(lex(test_new_lines), vec![create_token_number(2, 0)]);
    }

    #[test]
    fn lex_4_3() {
        let test_new_lines = "
// some comment
0
// some comment
// more comment";
        assert_eq!(lex(test_new_lines), vec![create_token_number(2, 0)]);
    }

    #[test]
    fn lex_4_4() {
        let test_new_lines = "
/*
Very unclear comment
*/
0";
        assert_eq!(lex(test_new_lines), vec![create_token_number(4, 0)]);
    }

    #[test]
    fn lex_4_5() {
        assert_eq!(lex("/**/"), Vec::new());
    }
    #[test]
    fn lex_4_6() {
        assert_eq!(lex("//"), Vec::new());
    }

    #[test]
    fn lex_5() {
        let number_and_unit = "256K";
        assert_eq!(
            lex(number_and_unit),
            vec![create_token_number(0, 256), create_token_word(0, "K")]
        );
    }

    #[test]
    fn lex_6_1() {
        let number_and_unit = "256, 368";
        assert_eq!(
            lex(number_and_unit),
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
    fn lex_6_2() {
        let number_and_unit = "
256
,
368";
        assert_eq!(
            lex(number_and_unit),
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
    fn lex_7_1_1() {
        const LINKER_SCRIPT: &str = "MEMORY
        PROBLEMS
";

        let expected = vec![
            create_token_word(0, "MEMORY"),
            create_token_word(1, "PROBLEMS"),
        ];
        assert_eq!(lex(LINKER_SCRIPT), expected);
    }
    #[test]
    fn lex_7_1() {
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

        assert_eq!(lex(LINKER_SCRIPT), expected);
    }

    #[test]
    fn lex_7_2() {
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

        assert_eq!(lex(LINKER_SCRIPT), expected);
    }
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
