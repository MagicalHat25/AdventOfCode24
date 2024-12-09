use std::fs;

// Reads the file and splits it into lines
fn read_file_lines(input_file_path: &str) -> Vec<String> {
    fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

// Parses a line into a vector of integers
fn parse_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect()
}

fn check_if_safe(list: &mut Vec<i32>) -> bool {
    let mut safe = true;
    let mut increasing = true;

    // Initial safety check
    for (i, window) in list.windows(2).enumerate() {
        if let [current, next] = window {
            if current == next || (current - next).abs() >= 4 {
                safe = false;
            }
            if i == 0 {
                increasing = current < next;
            } else {
                if increasing && current > next || !increasing && current < next {
                    safe = false;
                }
            }
        }
    }

    // If not safe, try removing one element
    if !safe {
        for index in 0..list.len() {
            // Remove the element
            let removed_element = list.remove(index);

            // Reset safety and increasing flags
            safe = true;
            increasing = true;

            // Recheck the modified list
            for (i, window) in list.windows(2).enumerate() {
                if let [current, next] = window {
                    if current == next || (current - next).abs() >= 4 {
                        safe = false;
                    }
                    if i == 0 {
                        increasing = current < next;
                    } else {
                        if increasing && current > next || !increasing && current < next {
                            safe = false;
                        }
                    }
                }
            }

            if safe {
                return true; // The list is safe after removal
            }

            // If still not safe, reinsert the removed element
            list.insert(index, removed_element);
        }

        return false; // Still not safe after trying all single-element removals
    }

    safe
}


// Main function logic
pub fn process(input_file_path: &str) {
    let lines = read_file_lines(input_file_path);

    let mut number_of_safe_lists = 0;

    for line in lines {
        let mut numbers = parse_numbers(&line);

        if(check_if_safe(&mut numbers)) { number_of_safe_lists += 1; }
    }

    println!("Number of safe lists: {}", number_of_safe_lists);
}
