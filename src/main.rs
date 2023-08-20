use std::env;
use std::fs;
use std::process;

fn format_json(json_string: &str) -> String {
    let mut formatted = String::new();
    let mut indent = 0;
    for ch in json_string.chars().filter(|c| !c.is_whitespace()) {
        match ch {
            '{' | '[' => {
                formatted.push(ch);
                formatted.push('\n');
                indent += 4;
                formatted.push_str(&" ".repeat(indent));
            }
            '}' | ']' => {
                formatted.push('\n');
                indent -= 4;
                formatted.push_str(&" ".repeat(indent));
                formatted.push(ch);
            }
            ',' => {
                formatted.push(ch);
                formatted.push('\n');
                formatted.push_str(&" ".repeat(indent));
            }
            ':' => {
                formatted.push(ch);
                formatted.push(' ');
            }
            _ => formatted.push(ch),
        }
    }
    formatted
}

fn minify_json(json_string: &str) -> String {
    json_string.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} [-m] <filename>", args[0]);
        process::exit(1);
    }

    let (filename, minify) = if args.len() == 3 && args[1] == "-m" {
        (&args[2], true)
    } else {
        (&args[1], false)
    };

    let json_string = fs::read_to_string(filename)?;

    if minify {
        println!("{}", minify_json(&json_string));
    } else {
        println!("{}", format_json(&json_string));
    }

    Ok(())
}
