use std::env;
use std::fs;
use std::process;

fn format_json(json_string: &str) -> String {
    let mut formatted = String::new();
    let mut indent = 0;
    let mut in_string = false;
    let mut was_backslash = false;
    for ch in json_string.chars() {
        if in_string {
            formatted.push(ch);
            if ch == '"' && !was_backslash {
                in_string = false;
            }
            was_backslash = ch == '\\' && !was_backslash;
        } else {
            if ch == '"' {
                in_string = true;
                formatted.push(ch);
            } else if !ch.is_whitespace() {
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
        }
    }
    formatted
}

fn minify_json(json_string: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut was_backslash = false;
    for ch in json_string.chars() {
        if in_string {
            result.push(ch);
            if ch == '"' && !was_backslash {
                in_string = false;
            }
            was_backslash = ch == '\\' && !was_backslash;
        } else {
            if ch == '"' {
                in_string = true;
                result.push(ch);
            } else if !ch.is_whitespace() {
                result.push(ch);
            }
        }
    }
    result
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
