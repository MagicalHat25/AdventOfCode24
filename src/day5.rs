use std::fs;
use std::str::FromStr;
use std::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError {
            message: error.to_string()
        }
    }
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            message: msg.to_string(),
        }
    }
}

pub fn parse_file(filename: &str) -> Result<(Vec<(u32, u32)>, Vec<Vec<u32>>), ParseError> {
    // Read the file contents into a string and normalize line endings
    let contents = fs::read_to_string(filename)?;
    let contents = contents.replace("\r\n", "\n");

    // Split into sections, being more lenient with empty lines
    let sections: Vec<&str> = contents
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if sections.len() != 2 {
        println!("Debug - Number of sections found: {}", sections.len());
        println!("Debug - Content:\n{}", contents);
        return Err(ParseError::new("File format incorrect: expected two sections separated by blank line"));
    }

    // Parse the first section (pairs)
    let pairs: Vec<(u32, u32)> = sections[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 2 {
                return Err(ParseError::new("Invalid pair format"));
            }

            let x = u32::from_str(parts[0].trim())
                .map_err(|_| ParseError::new("Failed to parse number"))?;
            let y = u32::from_str(parts[1].trim())
                .map_err(|_| ParseError::new("Failed to parse number"))?;
            Ok((x, y))
        })
        .collect::<Result<Vec<_>, ParseError>>()?;

    // Parse the second section (number lists)
    let number_lists: Vec<Vec<u32>> = sections[1]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|num| {
                    u32::from_str(num.trim())
                        .map_err(|_| ParseError::new("Failed to parse number"))
                })
                .collect::<Result<Vec<_>, ParseError>>()
        })
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok((pairs, number_lists))
}

fn validate_lists(pairs: &[(u32, u32)], number_lists: &[Vec<u32>]) -> Vec<bool> {
    let mut dependencies: HashMap<u32, HashSet<u32>> = HashMap::new();

    for &(before, after) in pairs {
        dependencies
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }

    number_lists
        .iter()
        .map(|list| validate_single_list(list, &dependencies))
        .collect()
}

fn validate_single_list(list: &[u32], dependencies: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, &current_num) in list.iter().enumerate() {
        if let Some(must_come_after) = dependencies.get(&current_num) {
            let numbers_before: HashSet<_> = list[..i].iter().collect();

            if must_come_after.iter().any(|after| numbers_before.contains(after)) {
                return false;
            }
        }
    }
    true
}

fn build_graph(pairs: &[(u32, u32)], numbers: &[u32]) -> (HashMap<u32, Vec<u32>>, HashMap<u32, usize>) {
    // Two things we're creating:
    // 1. graph: shows what each number must come before
    // 2. in_degree: counts how many numbers must come before each number
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // First, make sure every number is in our maps, even if it has no rules
    for &num in numbers {
        graph.entry(num).or_default();  // Add empty Vec if number isn't there
        in_degree.insert(num, 0);       // Start with 0 numbers that must come before it
    }

    // Now, for each rule like (47, 53) meaning "47 must come before 53":
    for &(before, after) in pairs {
        if numbers.contains(&before) && numbers.contains(&after) {
            // Add 'after' to the list of numbers that must come after 'before'
            graph.entry(before).or_default().push(after);
            // Increment count of numbers that must come before 'after'
            *in_degree.entry(after).or_default() += 1;
        }
    }

    (graph, in_degree)
}

fn topological_sort(pairs: &[(u32, u32)], numbers: &[u32]) -> Option<Vec<u32>> {
    let (graph, mut in_degree) = build_graph(pairs, numbers);
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    // Find all numbers that don't need anything before them
    for &num in numbers {
        if in_degree[&num] == 0 {
            queue.push_back(num);
        }
    }

    // While we have numbers we can place:
    while let Some(current) = queue.pop_front() {
        // Add this number to our result
        result.push(current);

        // Look at all numbers that needed this number before them
        if let Some(neighbors) = graph.get(&current) {
            for &next in neighbors {
                // This number is placed, so decrease the count for numbers waiting on it
                *in_degree.get_mut(&next).unwrap() -= 1;
                // If a number no longer needs any numbers before it, we can place it
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    // If we placed all numbers, we found a valid order!
    if result.len() == numbers.len() {
        Some(result)
    } else {
        None
    }
}

fn get_middle_number(list: &[u32]) -> u32 {
    let middle_index = list.len() / 2;
    list[middle_index]
}

pub fn process(input_file_name: &str) -> Result<(), ParseError> {
    let (pairs, number_lists) = parse_file(input_file_name)?;
    let results = validate_lists(&pairs, &number_lists);

    let mut sum_of_reordered_middles = 0;

    println!("Processing lists:");
    for (i, (list, is_valid)) in number_lists.iter().zip(results.iter()).enumerate() {
        println!("\nList {}: {:?}", i + 1, list);
        println!("Initially valid: {}", is_valid);

        if !is_valid {
            if let Some(reordered) = topological_sort(&pairs, list) {
                println!("Reordered: {:?}", reordered);

                // Validate the reordered list
                if validate_single_list(&reordered, &dependencies_map(&pairs)) {
                    let middle = get_middle_number(&reordered);
                    println!("Middle number after reordering: {}", middle);
                    sum_of_reordered_middles += middle;
                } else {
                    println!("Warning: Reordered list is still invalid!");
                }
            } else {
                println!("Warning: Could not reorder list (circular dependencies detected)");
            }
        }
    }

    println!("\nSum of middle numbers from successfully reordered lists: {}",
             sum_of_reordered_middles);

    Ok(())
}

fn dependencies_map(pairs: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut dependencies = HashMap::new();
    for &(before, after) in pairs {
        dependencies
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }
    dependencies
}