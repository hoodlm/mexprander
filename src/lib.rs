use std::str::FromStr;

pub fn expand(expression: String) -> Vec<String> {
    let tokens = tokenize(&expression);
    return compile_from_flat_list(&tokens);
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Plus,
    Multiply,
    Number(i64),
}

fn tokenize(expression: &str) -> Vec<Token> {
    return expression
        .split_whitespace()
        .map(|x| to_token(x))
        .collect();
}

fn to_token(token_str: &str) -> Token {
    match token_str {
        "+" => Token::Plus,
        "*" => Token::Multiply,
        n if n.chars().all(|c| c.is_ascii_digit()) => Token::Number(i64::from_str(n).unwrap()),
        x => panic!("Unknown character {}", x),
    }
}

enum CompilerSM {
    NeedLHS,
    NeedOperator,
    NeedRHS,
}

fn compile_from_flat_list(tokens: &Vec<Token>) -> Vec<String> {
    let mut output = Vec::new();

    let var_table = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't'];

    let mut var_picker = var_table.iter();
    let mut current_variable = var_picker.next().unwrap();
    let mut previous_variable = current_variable;
    let mut current_command = format!("{} = ", current_variable);

    let mut state = CompilerSM::NeedLHS;
    for token in tokens {
        match state {
            CompilerSM::NeedLHS => {
                match token {
                    Token::Number(x) => {
                        current_command.push_str(&x.to_string());
                        state = CompilerSM::NeedOperator;
                    },
                    _ => panic!("Compiler state is NeedLHS, next token must be a Number"),
                }
            },
            CompilerSM::NeedOperator => {
                match token {
                    Token::Multiply => {
                        current_command.push_str(" * ");
                        state = CompilerSM::NeedRHS;
                    },
                    Token::Plus => {
                        current_command.push_str(" + ");
                        state = CompilerSM::NeedRHS;
                    },
                    _ => panic!("Compiler state is NeedOperator, next token must be a Plus"),
                }
            },
            CompilerSM::NeedRHS => {
                match token {
                    Token::Number(x) => {
                        current_command.push_str(&x.to_string());

                        // Command is done... dump it to output
                        output.push(current_command);

                        // Previous result will be the LHS of the next command
                        previous_variable = current_variable;
                        current_variable = var_picker.next().unwrap();

                        // And start the next one
                        current_command = format!("{} = {}", current_variable, previous_variable);

                        // Finally, move the state machine forward
                        state = CompilerSM::NeedOperator;
                    }
                    _ => panic!("Compiler state is NeedRHS, next token must be a Number"),
                }
            }
        }
    }
    // Finally, evaluate the last variable:
    output.push(previous_variable.to_string());

    return output;
}

/*
struct TokenNode<'a> {
    this_token: Option<Token>,
    children: Vec<&'a TokenNode<'a>>,
}

fn to_tree(tokens: &Vec<Token>) -> TokenNode {
    let root = TokenNode { this_token: None, children: Vec::new() };
    return root;
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_no_paren() {
        let input = String::from("1 + 2");
        let result = expand(input);

        let mut expected = Vec::new();
        expected.push("a = 1 + 2");
        expected.push("a");

        assert_eq!(expected, result);
    }

    #[test]
    fn add_three_no_paren() {
        let input = String::from("1 + 2 + 3");
        let result = expand(input);

        let mut expected = Vec::new();
        expected.push("a = 1 + 2");
        expected.push("b = a + 3");
        expected.push("b");

        assert_eq!(expected, result);
    }

    #[test]
    fn add_multiply_no_paren() {
        let input = String::from("6 * 7 + 1");
        let result = expand(input);

        let mut expected = Vec::new();
        expected.push("a = 6 * 7");
        expected.push("b = a + 1");
        expected.push("b");

        assert_eq!(expected, result);
    }

    #[test]
    fn add_many_no_paren() {
        let input = String::from("100 + 200 + 300 + 1 + 2 + 3 + 4 + 5 + 7 + 4 + 2 + 1");
        let result = expand(input);

        let mut expected = Vec::new();
        expected.push("a = 100 + 200");
        expected.push("b = a + 300");
        expected.push("c = b + 1");
        expected.push("d = c + 2");
        expected.push("e = d + 3");
        expected.push("f = e + 4");
        expected.push("g = f + 5");
        expected.push("h = g + 7");
        expected.push("i = h + 4");
        expected.push("j = i + 2");
        expected.push("k = j + 1");
        expected.push("k");

        assert_eq!(expected, result);
    }

    #[test]
    fn tokenize_add_three_no_paren() {
        let input = String::from("1 + 2 + 3");
        let result = tokenize(&input);

        let mut expected = Vec::new();
        expected.push(Token::Number(1));
        expected.push(Token::Plus);
        expected.push(Token::Number(2));
        expected.push(Token::Plus);
        expected.push(Token::Number(3));

        assert_eq!(expected, result);
    }

    #[test]
    fn tokenize_multiply_three_no_paren() {
        let input = String::from("4 * 8 * 16");
        let result = tokenize(&input);

        let mut expected = Vec::new();
        expected.push(Token::Number(4));
        expected.push(Token::Multiply);
        expected.push(Token::Number(8));
        expected.push(Token::Multiply);
        expected.push(Token::Number(16));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn tokenize_with_unknown_symbol() {
        let input = String::from("1 + FOO + 3");
        tokenize(&input);
    }

    #[test]
    fn token_plus_sign() {
        assert_eq!(Token::Plus, to_token("+"));
    }

    #[test]
    fn token_multiply_star() {
        assert_eq!(Token::Multiply, to_token("*"));
    }

    #[test]
    fn token_single_digit() {
        assert_eq!(Token::Number(5), to_token("5"));
    }

    #[test]
    fn token_multi_digit() {
        assert_eq!(Token::Number(1357), to_token("1357"));
    }

    #[test]
    #[should_panic]
    fn token_unknown_symbol() {
        to_token("foo");
    }
}
