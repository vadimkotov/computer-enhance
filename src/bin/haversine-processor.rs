mod json;

use std::env;
use std::io::Read;
use std::fs::File;

fn process_json(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?; 
    let mut data = String::new();

    // TODO: use buffered i/o for large files?
    file.read_to_string(&mut data)?;

    let tokens = json::tokenize(&data);
    json::parse(&tokens);

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
