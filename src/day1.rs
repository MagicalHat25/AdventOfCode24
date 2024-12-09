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

// Distributes numbers into two lists
fn split_numbers(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for (i, num) in numbers.into_iter().enumerate() {
        if i % 2 == 0 {
            first_list.push(num);
        } else {
            second_list.push(num);
        }
    }

    (first_list, second_list)
}

// Calculates the sum of absolute differences between two sorted lists
fn calculate_sum_of_differences(first_list: &[i32], second_list: &[i32]) -> i32 {
    first_list
        .iter()
        .zip(second_list)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

// Calculates similarity based on occurrences
fn calculate_similarity(first_list: &[i32], second_list: &[i32]) -> i32 {
    first_list
        .iter()
        .map(|num| {
            let occurrences = second_list.iter().filter(|&&n| n == *num).count();
            *num as usize * occurrences
        })
        .sum::<usize>() as i32
}

// Main function logic
pub fn process(input_file_path: &str) {
    let lines = read_file_lines(input_file_path);

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in lines {
        let numbers = parse_numbers(&line);
        let (mut first, mut second) = split_numbers(numbers);
        first_list.append(&mut first);
        second_list.append(&mut second);
    }

    first_list.sort();
    second_list.sort();

    let sum_of_differences = calculate_sum_of_differences(&first_list, &second_list);
    println!("The sum of the differences is: {}", sum_of_differences);

    let similarity = calculate_similarity(&first_list, &second_list);
    println!("The similarity of the two lists is: {}", similarity);
}
