use std::fs;
use regex::Regex;

fn find_and_compute_multiplications(input: &str) -> i32 {
    let mut result = 0;
    let mut enable = true;

    // Compile regex patterns
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enable_regex = Regex::new(r"do\(\)").unwrap();
    let disable_regex = Regex::new(r"don't\(\)").unwrap();

    let patterns = [
        ("disable", &disable_regex),
        ("enable", &enable_regex),
        ("mul", &mul_regex),
    ];

    let mut pos = 0;

    while pos < input.len() {
        // Find the earliest match among all patterns
        let mut earliest_match: Option<(&str, usize, usize)> = None;

        for (label, regex) in &patterns {
            if let Some(mat) = regex.find(&input[pos..]) {
                let start = pos + mat.start();
                let end = pos + mat.end();

                if earliest_match.is_none() || start < earliest_match.unwrap().1 {
                    earliest_match = Some((label, start, end));
                }
            }
        }

        if let Some((label, start, end)) = earliest_match {
            match label {
                "enable" => { enable = true; }
                "disable" => { enable = false; }
                "mul" => {
                    if enable {
                        if let Some(caps) = mul_regex.captures(&input[start..end]) {
                            let a: i32 = caps[1].parse().unwrap();
                            let b: i32 = caps[2].parse().unwrap();
                            result += a * b;
                        }
                    }
                }
                _ => {}
            }
            // Move the position to the end of the current match
            pos = end;
        } else {
            // No match found, move to the next character
            pos += 1;
        }
    }

    result
}

pub fn process(input_file_path : &str) {
    let string = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let result = find_and_compute_multiplications(&string);

    println!("{}", result);
}