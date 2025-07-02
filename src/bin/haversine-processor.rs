use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::fs::File;

#[derive(Debug)]
enum Token<'a> {
    BeginObj,  // '{'
    EndObj,    // '}'
    BeginArr,  // '['
    EndArr,    // ']'
    Colon,
    Comma,

    Str(&'a str),
    Num(&'a str),
}

/*
 * NOTE
 *  - No support for escaped characters '\"'
 */
fn take_string(data: &str) -> (&str, &str) {
    // Opening quote is not consumed.
    for (i, ch) in data.char_indices().skip(1) {
        if ch == '"' {
            // Consume the closing quote.
            return (&data[1..i], &data[i+1..]);
        }
    }
    panic!("Unterminated quote");
}

fn take_number(data: &str) -> (&str, &str) {
    // We don't check the format of a number
    // it will be done at the parsing stage.
    let mut end = data.len();

    for (i, ch) in data.char_indices() {
        match ch {
            '0'..='9' | '-' | '.' | '+' | 'e' | 'E' => {},
            _ => { 
                end = i;
                break;
            },
        }
    }
    (&data[..end], &data[end..])
}

// TODO: report error?
fn tokenize(mut data: &str) -> Vec<Token> {

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(ch) = data.chars().next() {
        match ch {
            c if c.is_whitespace() => {
                data = &data[1..];
            },
            '{' => { 
                tokens.push(Token::BeginObj);
                data = &data[1..];
            },
            '}' => {
                tokens.push(Token::EndObj);
                data = &data[1..];
            },
            ':' => {
                tokens.push(Token::Colon);
                data = &data[1..];
            },
            ',' => {
                tokens.push(Token::Comma);
                data = &data[1..];
            },
            '[' => {
                tokens.push(Token::BeginArr);
                data = &data[1..];
            },
            ']' => {
                tokens.push(Token::EndArr);
                data = &data[1..];
            },
            '"' => {
                let (s, rest) = take_string(data);
                // println!("parse_string: {}", s);
                tokens.push(Token::Str(s));
                data = rest
            },
            '0'..='9' | '-' => {
                let (n, rest) = take_number(data);
                // println!("parse_number: {}", n);
                tokens.push(Token::Num(n));
                data = rest;
            },
            _ => {
                println!("Unimplemented: {}", ch);
                break;
            }
        }
    }
    tokens
}

struct Cursor<'a> {
    tokens: &'a[Token<'a>],
    pos: usize,
}

#[derive(Debug)]
enum JsonValue<'a> {
    Num(f64), // NOTE: for simplicity we treat all numbers as f64!
    Array(Vec<JsonValue<'a>>),
    // NOTE: &str will probably need to change if we ever decide to support escaped characters.
    Object(HashMap<&'a str, JsonValue<'a>>),
}

impl<'a> Cursor<'a> {
    fn bump(&mut self) -> Option<&Token<'a>> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1; 
        tok
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    fn expect(&mut self, wanted: &Token<'_>) {
        match self.bump() {
            Some(tok) if std::mem::discriminant(wanted) == std::mem::discriminant(tok) => {}, 
            // TODO: improve error message, probably implement std::fmt::Display trait?
            _ => panic!("Expected different token"),
        }
    }
}

fn parse_object<'a>(cursor: &mut Cursor<'a>) -> Result<JsonValue<'a>, &'static str> {
    /* object
        '{' members '}'
    */
    // Therefore we need to parse an arbitrary number of members.
    let mut map = HashMap::new();
    loop {
        // string ':' element
        let key = match cursor.bump() {
            Some(&Token::Str(s)) => s,
            _ => return Err("Ran out of tokens when parsing object name"),
        };
        cursor.expect(&Token::Colon);
        let val = parse_value(cursor)?;
        map.insert(key, val); 
        match cursor.peek() {
            Some(Token::Comma) => { cursor.bump(); },
            Some(Token::EndObj) => { cursor.bump(); break; },
            _ => return Err("Expected comma or '}'"),
        }
            
    }
    Ok(JsonValue::Object(map))
}

fn parse_array<'a>(cursor: &mut Cursor<'a>) -> Result<JsonValue<'a>, &'static str> {
    /* array
        '[' elements ']'

       elements
        element
        element ',' elements
    */
    let mut arr = Vec::new();
    loop {
        let val = parse_value(cursor)?;
        arr.push(val);
        match cursor.peek() {
            Some(Token::Comma) => { cursor.bump(); },
            Some(Token::EndArr) => { cursor.bump(); break; },
            _ => return Err("Expected comma or ']'"),
        }
    }
    Ok(JsonValue::Array(arr))
}

fn parse_value<'a>(cursor: &mut Cursor<'a>) -> Result<JsonValue<'a>, &'static str> {
   // dbg!(cursor.peek());
   match cursor.bump() {
       Some(Token::BeginObj) => parse_object(cursor),
       Some(Token::BeginArr) => parse_array(cursor),
       Some(Token::Num(s)) => Ok(JsonValue::Num(s.parse::<f64>().map_err(|_| "Bad f64")?)),
       // TODO: provide info about the bad token
       _ => Err("Bad token"), 
   }
}


fn parse(tokens: &[Token]) {
    let mut cursor = Cursor{tokens: tokens, pos: 0};
    let res = parse_value(&mut cursor);
}


fn process_json(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?; 
    let mut data = String::new();

    // TODO: use buffered i/o for large files?
    file.read_to_string(&mut data)?;

    let tokens = tokenize(&data);
    parse(&tokens);

    Ok(())
}

fn main() {
    let mut args =  env::args().skip(1);
    let mut json_path: Option<String> = None;
    let mut _floats_path: Option<String> = None;

    while let Some(argstr) = args.next() {
        if argstr == "-j" {
            json_path = args.next();
        } else if argstr == "-f" {
            _floats_path = args.next();
        } else {
            println!("Unexpected flag: {}", argstr);
            return;
        }
    }

    if let Some(json_path) = json_path {
        match process_json(&json_path) {
            Ok(_) => println!("Done"),
            Err(why) => println!("Error: {}", why),
        }
    }
}
