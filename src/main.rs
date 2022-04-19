fn main() {
    println!("Hello, world!");
}
#[derive(Debug, PartialEq)]
struct Token {
    line_number: usize,
    token_type: TokenType,
}
#[derive(Debug, PartialEq)]
enum TokenType {
    Plus,
    Number(u64),
    Word(String),
}
fn lex(script: &str) -> Vec<Token> {
    let mut vec_tokens = Vec::new();

    let mut line_number = 0;

    //let mut it = script.chars().peekable();
    let mut it = script.chars().enumerate().peekable();
    while let Some((ind, ch)) = it.next() {
        println!("this is ch: {}", ch);
        match ch {
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

                // we need to know how long the number is
                // that returns the position of the first non-hex-digit char in the input
                let stop = it
                    .by_ref()
                    .inspect(|(ind, ch)| println!("about to take_while: {}, on index {}", ch, ind))
                    .find(|(_, ch)| !ch.is_ascii_hexdigit())
                    .map(|(idx, _)| idx)
                    .unwrap_or(script.len());

                println!("radix: {}", radix);
                let number = u64::from_str_radix(&script[start..stop], radix).unwrap();

                vec_tokens.push(Token {
                    line_number,
                    token_type: TokenType::Number(number),
                });
            }
            'a'..='z' | 'A'..='Z' => {
                println!("char in 'a'..='z' | 'A'..='Z': {}", ch);
                let mut string = String::new();
                string.push(ch);

                it.by_ref()
                    .take_while(|(_, c)| c.is_alphabetic())
                    .for_each(|(_, c)| string.push(c));

                println!("string in 'a'..='z' {}", string);

                vec_tokens.push(Token {
                    line_number,
                    token_type: TokenType::Word(string),
                })
            }
            _ => {
                continue;
            }
        }
    }

    vec_tokens
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
