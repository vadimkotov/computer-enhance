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
 * WARNING:
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

fn parse(tokens: &[Token]) {

}


fn process_json(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?; 
    let mut data = String::new();

    // TODO: use buffered i/o for large files?
    file.read_to_string(&mut data)?;

    let tokens = tokenize(&data);

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
