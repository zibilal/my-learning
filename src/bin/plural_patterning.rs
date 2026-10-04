use std::vec::IntoIter;

fn main() {
    let opt_val = Some(String::from("hello"));
    if let Some(ref s) = opt_val {
        println!("{}", s); // s is &String
    }

    // rust 2018
    if let Some(s) = &opt_val {
        println!("{}", s); // s is &String
    }

    // ref mut
    let mut opt_val = Some(String::from("world"));

    if let Some(ref mut s) = opt_val {
        s.push_str("!");
    }

    // rust 2018
    if let Some(s) = &mut opt_val {
        println!("{}", s);
    }

    println!("{}", opt_val.unwrap());

    // Advanced Guard and Bindings
    // Guards add conditions to matching
    let number = 42;
    match number {
        n if n % 2 == 0 => println!("{} is even", n),
        n if n % 2 != 0 => println!("{} is odd", n),
        _ => println!("Unexpected case"),
    }


    // The problem with if-let chains
    // If the if-let chain is verbose, deeply nested, and harder to maintain
    // especially for complex types.
    let data: Option<Result<i32, String>> = Some(Ok(15));
    if let Some(Ok(num)) = &data {
        if *num > 0 {
            println!("Positive number: {}", num);
        } else {
            println!("Non-positive number: {}", num);
        }
    } else if let Some(Err(err)) = &data {
        println!("Error: {}", err);
    } else {
        println!("No data");
    }

    match &data {
        Some(Ok(num)) if *num > 0 => println!("Positive number: {}", num),
        Some(Ok(num)) => println!("Non-positive number: {}", num),
        Some(Err(err)) => println!("Error: {}", err),
        None => println!("No data"),
    }


}

// setting up and configuring the HttpClint begins inside
fn check_number(input: Option<Result<i32, &'static str>>) -> &'static str {
    match input {
        Some(Ok(num)) => "Got a number",
        Some(Err(_)) => "Got an error",
        None => "No input",
    }
}

// Demo 1
enum ParseNode {
    Number(i32),
    String(String),
    Object(Vec<(String, ParseNode)>),
}

// first implementation
pub fn first_parse_nested_object(input: ParseNode) -> Option<(String, i32)> {
    match input {
        ParseNode::Object(fields) => {
            for (key, val) in fields {
                match val {
                    ParseNode::Number(num) => return Some((key, num)),
                    _ => continue,
                }
            }
            None
        }
        _ => None
    }
}

// second implementation
pub fn second_parse_nested_object(input: ParseNode) -> Option<(String, i32)> {
    match input {
        ParseNode::Object(fields) =>
        fields
            .into_iter()
            .find_map(|(key, value)| match value {
                ParseNode::Number(num) => Some((key.to_string(), num)),
                _ => None
            }),
        _ => None
    }
}

// Demo patterns and slices and iterators
#[derive(Debug, Clone)]
enum Token {
    Keyword(&'static str),
    Number(i32),
}

pub fn parse_tokens(tokens: &[Token]) -> Option<(Token, Vec<Token>)> {
    match tokens {
        [first, rest @ ..] => Some((first.clone(), rest.to_vec())),
        _ => None,
    }
}

pub fn parse_iterator(tokens: Vec<Token>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut iter: IntoIter<Token> = tokens.into_iter();

    while let Some(Token::Number(n)) = iter.next() {
        numbers.push(n);
    }
    numbers
}

// Demo Guarded Matching
enum TokenAgain {
    Keyword(&'static str),
    Number(i32),
    Identifier(String),
}

pub fn parse_declaration(tokens: &[TokenAgain]) -> Option<String> {
    match tokens {
        [TokenAgain::Keyword(left), TokenAgain::Identifier(name), TokenAgain::Number(val)]
        if !name.is_empty() && *val >= 0 => Some(format!("Declared {} = {}", name, val)),
        _ => None,
    }
}

enum AnotherToken {
    Keyword(&'static str),
    Number(i32),
}

pub fn first_process_input(input: Option<Result<i32, &'static str>>) -> String {
    let mut message: String = String::new();

    if let Some(result) = input {
        if let Ok(num) = result {
            if num > 0 {
                message = format!("Positive: {}", num);
            } else {
                message = format!("Non-positive: {}", num);
            }
        } else if let Err(err) = result {
            message = format!("Error: {}", err);
        }
    } else {
        message = "No data".to_string();
    }

    message
}

pub fn second_process_input(input: Option<Result<i32, &'static str>>) -> String {
    let message: String;
    match input {
        Some(Ok(num)) if num > 0 => message = format!("Positive: {}", num),
        Some(Ok(num)) => message = format!("Non-positive: {}", num),
        Some(Err(err)) => message = format!("Error: {}", err),
        None => message = "No data".to_string(),
    }
    message
}
#[derive(PartialEq)]
enum TokenParser {
    Keyword(&'static str),
    Number(i32),
    End,
}

pub fn parser_process_tokens(tokens: Vec<TokenParser>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut iter:IntoIter<TokenParser> = tokens.into_iter();

    while let Some(token) = iter.next() {
        if token == TokenParser::End {
            break;
        }
        if let TokenParser::Number(num) = token {
            if num > 0 {
                numbers.push(num);
            }
        }
    }
    numbers
}

pub fn parser2_process_tokens(tokens: Vec<TokenParser>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut iter: IntoIter<TokenParser> = tokens.into_iter();

    while let Some(token) = iter.next() {
        match token {
            TokenParser::End => break,
            TokenParser::Number(num) if num > 0 => numbers.push(num),
            _ => continue,
        }
    }

    numbers
}