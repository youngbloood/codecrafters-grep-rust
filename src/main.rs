use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    // match digits
    if pattern.eq("\\d") {
        if input_line.len() == 0 {
            return false;
        }
        let digital = input_line.parse::<i128>();
        if digital.is_err() {
            return false;
        }
        return true;
    }
    // match alphanumeric
    // (a-z, A-Z, 0-9, _)
    if pattern.eq("\\w") {
        if input_line.len() == 0 {
            return false;
        }
        let first = *input_line.as_bytes().into_iter().next().unwrap();
        if (first >= 'a' as u8 && first <= 'z' as u8)
            || (first >= 'A' as u8 && first <= 'Z' as u8)
            || (first >= '0' as u8 && first <= '9' as u8)
            || first == '_' as u8
        {
            return true;
        }
        return false;
    }

    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
