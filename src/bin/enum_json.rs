use std::collections::HashMap;
use std::fmt::Write;

fn main() {
    let mut user = HashMap::new();
    user.insert("name".to_string(), Json::String("Ada \"The Countess\"".into()));
    user.insert("age".to_string(), Json::Number(36.0));
    user.insert("admin".to_string(), Json::Boolean(true));
    user.insert("manager".to_string(), Json::Null);
    user.insert("scores".to_string(), Json::Array(vec![Json::Number(9.5), Json::Number(7.0)]));

    let value = Json::Object(Box::new(user));
    let text = marshal(&value);
    println!("marshaled: {}", text);

    // Unmarshal it back
    let parsed = unmarshal(&text).expect("not valid JSON");
    println!("unmarshaled: {:?}", parsed);
    assert_eq!(parsed, value);

    // Unmarshal from a raw string, including whitespace and escape
    let raw = r#" { "greeting": "hi\n\u00e9", "list": [1, 2.5, -3e2, null] } "#;
    match unmarshal(raw) {
        Ok(v) => println!("raw parsed: {}", marshal(&v)),
        Err(e) => eprintln!("error: {}", e),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

fn marshal(value: &Json) -> String {
    let mut out = String::new();
    write_value(value, &mut out);
    out
}

fn write_value(value: &Json, out: &mut String) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Boolean(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Number(n) if n.is_finite() => write!(out, "{}", n).unwrap(),
        Json::Number(_) => out.push_str("null"),
        Json::String(s) => write_string(s, out),
        Json::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        Json::Object(map) => {
            out.push('{');
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(key, out);
                out.push(':');
                write_value(&map[*key], out);
            }
            out.push('}');
        }
    }
}

fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
            c => out.push(c),
        }
    }
    out.push('"');
}



fn unmarshal(input: &str) -> Result<Json, String> {
    let mut p = Parser { chars: input.chars().collect(), pos: 0 };
    let value = p.parse_value()?;
    p.skip_ws();
    if p.pos != p.chars.len() {
        return Err(format!("trailing characters at position {}", p.pos));
    }

    Ok(value)
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ' | '\n' | '\r' | '\t')) {
            self.pos += 1;
        }
    }

    fn expect(&mut self, want: char) -> Result<(), String> {
        match self.next() {
            Some(c) if c == want => Ok(()),
            Some(c) => Err(format!("expected '{}', found '{}' at {}", want, c, self.pos - 1)),
            None => Err(format!("expected '{}', found end of input", want)),
        }
    }

    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_ws();
        match self.peek() {
            Some('n') => self.parse_literal("null", Json::Null),
            Some('t') => self.parse_literal("true", Json::Boolean(true)),
            Some('f') => self.parse_literal("false", Json::Boolean(false)),
            Some('"') => Ok(Json::String(self.parse_string()?)),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some(c) if c == '-' || c.is_ascii_digit() => self.parse_number(),
            Some(c) => Err(format!("unexpected '{}' at {}", c, self.pos)),
            None => Err("unexpected end of input".into()),
        }
    }

    fn parse_literal(&mut self, word: &str, value: Json) -> Result<Json, String> {
        for expected in word.chars() {
            if self.next() != Some(expected) {
                return Err(format!("invalid literal, expected '{}'", word));
            }
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || "+-.eE".contains(c)) {
            self.pos += 1;
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        text.parse::<f64>()
            .map(Json::Number)
            .map_err(|_| format!("invalid number '{}'", text))
    }

    fn parse_hex4(&mut self) -> Result<u32, String> {
        let mut n = 0;
        for _ in 0..4 {
            let c = self.next().ok_or("unterminated \\u escape")?;
            n = n * 16 + c.to_digit(16).ok_or("invalid hex digit in \\u escape")?;
        }
        Ok(n)
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect('"')?;
        let mut s = String::new();
        loop {
            match self.next().ok_or("unterminated string")? {
                '"' => return Ok(s),
                '\\' => match self.next().ok_or("unterminated escape")? {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\u{08}'),
                    'f' => s.push('\u{0C}'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => {
                        let mut code = self.parse_hex4()?;
                        // Handle UTF-16 surrogate pairs (e.g. emoji)
                        if (0xD800..0xDC00).contains(&code) {
                            if self.next() != Some('\\') || self.next() != Some('u') {
                                return Err("expected low surrogate".into());
                            }
                            let low = self.parse_hex4()?;
                            code = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                        }
                        s.push(char::from_u32(code).ok_or("invalid unicode escape")?);
                    }
                    c => return Err(format!("invalid escape '\\{}'", c)),
                },
                c => s.push(c),
            }

        }
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.expect('[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(Json::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.next() {
                Some(',') => continue,
                Some(']') => return Ok(Json::Array(items)),
                _ => return Err("expected ',' or ']' in array".into()),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.expect('{')?;
        let mut map = HashMap::new();
        self.skip_ws();
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(Json::Object(Box::new(map)));
        }
        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect(':')?;
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_ws();
            match self.next() {
                Some(',') => continue,
                Some('}') => return Ok(Json::Object(Box::new(map))),
                _ => return Err("expected ',' or '|' in object".into()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unmarshal() {
        let raw = r#"{ "greeting": "hi\n\u00e9", "list": [1, 2.5, -3e2, null] }"#;
        match unmarshal(raw) {
            Ok(v) => println!("raw parsed: {}", raw),
            Err(e) => eprintln!("error: {}", e),
        }
    }
}