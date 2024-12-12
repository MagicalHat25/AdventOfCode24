use std::{fs, io};
use std::io::BufRead;
use std::str::FromStr;

fn read_file_lines(input_file_path: &str) -> Vec<String> {
    fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

fn parse_test_line(line: &str) -> Result<(Vec<i64>, Vec<i64>), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid format: missing colon".into())
    }

    let result = i64::from_str(parts[0].trim())?;
    let results = vec![result];

    let inputs: Result<Vec<i64>, _> = parts[1]
        .split_whitespace()
        .map(|s| i64::from_str(s))
        .collect();

    Ok((results, inputs?))
}

// New enum to represent our three operation types
#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn concatenate(a: i64, b: i64) -> Option<i64> {
    // Convert b to string to get its length
    let b_str = b.to_string();
    // Calculate the shift needed (10^length of b)
    let shift = 10_i64.checked_pow(b_str.len() as u32)?;
    // Multiply a by the shift and add b
    a.checked_mul(shift)?.checked_add(b)
}

fn evaluate_with_ops(nums: &[i64], ops: &[Operation]) -> Option<i64> {
    if nums.len() != ops.len() + 1 {
        return None;
    }

    // Start with first number
    let mut result = nums[0];

    // Process each operation left to right
    for i in 0..ops.len() {
        result = match ops[i] {
            Operation::Add => result.checked_add(nums[i + 1])?,
            Operation::Multiply => result.checked_mul(nums[i + 1])?,
            Operation::Concatenate => concatenate(result, nums[i + 1])?,
        };
    }

    Some(result)
}

fn is_valid_combination(nums: &[i64], target: i64) -> bool {
    if nums.len() == 1 {
        return nums[0] == target;
    }

    let num_ops = nums.len() - 1;
    // For each position, we now have 3 choices of operators
    let max_combinations = 3_i64.pow(num_ops as u32);

    for i in 0..max_combinations {
        let mut ops = Vec::with_capacity(num_ops);
        let mut n = i;
        // Convert number to base-3 to get our operations
        for _ in 0..num_ops {
            ops.push(match n % 3 {
                0 => Operation::Add,
                1 => Operation::Multiply,
                2 => Operation::Concatenate,
                _ => unreachable!(),
            });
            n /= 3;
        }
        ops.reverse(); // Reverse to maintain left-to-right order
    }

    false
}

pub fn process(input_file_path: &str) -> io::Result<()> {
    let mut lines = read_file_lines(input_file_path);
    let mut sum: i64 = 0;

    for line in &mut lines {
        match parse_test_line(line) {
            Ok((results, inputs)) => {
                let target = results[0];
                if is_valid_combination(&inputs, target) {
                    println!("Valid combination found for inputs {:?} to make {}", inputs, target);
                    sum += target;
                } else {
                    println!("No valid combination found for inputs {:?} to make {}", inputs, target);
                }
            }
            Err(e) => println!("Error parsing string: {}", e),
        }
    }

    println!("Total sum of valid results: {}", sum);
    Ok(())
}