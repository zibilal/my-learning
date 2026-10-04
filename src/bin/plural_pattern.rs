fn main() {
    // destructuring extracts values from complex types like enums and structs
    // without cloning or moving ownership
    let msg1 = Message::Move { x: 5, y: 10 };
    // let msg2 = Message::Write(String::from("hello"));

    match msg1 {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y} => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
    }

    let value = Top::Middle(Middle::Bottom(Bottom::Value(42)));
    match value {
        Top::Middle(Middle::Bottom(Bottom::Value(n))) =>
        println!("Value from nested enums: {:?}", value),

        Top::Middle(Middle::Bottom(Bottom::Data(data_string))) =>
        println!("Data from nested enums: {:?}", data_string),
    }

    let opt_val = Some(String::from("hello"));
    if let Some(ref s) = opt_val {
        println!("{}", s);
    }

    // rust 2018
    if let Some(s) = &opt_val {
        println!("{}", s); // s is &String
    }

    // ref mut
    let mut opt_val = Some(String::from("world"));
    if let Some(ref mut s) = opt_val {
        s.push_str("!");  // s is &mut String
        println!("{}", s);
    }
    if let Some(s)= &mut opt_val {
        s.push_str(" !!!");
        println!("{}", s);
    }
    println!("{}", opt_val.unwrap());
}

enum Message {
    Move { x: i32, y: i32 },
    Write(String),
    Quit,
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum Bottom {
    Value(i32),
    Data(String),
}
#[derive(Debug)]
enum Middle {
    Bottom(Bottom),
}
#[derive(Debug)]
enum Top {
    Middle(Middle),
}

fn check_number(input: Option<Result<i32, &'static str>>) -> &'static str {
    match input {
        Some(Ok(num)) => "Got a number",
        Some(Err(_)) => "Got an error",
        None => "No input",
    }
}

enum ParseNode {
    Number(i32),
    String(String),
    Object(Vec<(String, ParseNode)>),
}

// first implementation
pub fn parse_nested_object_1(input: ParseNode) -> Option<(String, i32)> {
    match input {
        ParseNode::Object(fields) => {
            for (key, value) in fields {
                match value {
                    ParseNode::Number(num) => return Some((key, num)),
                    _ => continue,
                }
            }
            None
        }
        _ => None,
    }
}

pub fn parse_nested_object_2(input: ParseNode) -> Option<(String, i32)> {
    match input {
        ParseNode::Object(fields) =>
        fields
            .into_iter()
            .find_map(|(key, value)| match value {
                ParseNode::Number(num) => Some((key, num)),
                _ => None,
            }),
        _ => None,
    }
}

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