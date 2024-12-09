use std::fs;
use std::collections::HashSet;

pub fn process(input_file_path: &str) {
    let string = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut unique_matches = HashSet::new();

    let rows: Vec<&str> = string.lines().collect();

    for line_index in 0..rows.len() {
        for char_index in 0..rows[line_index].len() {
            if let Some(match_location) = search_direction::x_pattern_mas(
                &rows,
                line_index,
                char_index,
                "MAS"
            ) {
                // Use just the start line and start char to create a unique key
                unique_matches.insert((match_location.start_line, match_location.start_char));
            }
        }
    }

    println!("Total unique MAS X pattern matches: {}", unique_matches.len());
}

// Module to encapsulate search direction functions
mod search_direction {
    use std::fmt;

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub struct MatchLocation {
        pub(crate) start_line: usize,
        pub(crate) start_char: usize,
        direction: &'static str,
    }

    // Each search function returns an Option<MatchLocation>
    pub fn forward(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        let line = rows[line_index];
        if char_index + match_word.len() > line.len() {
            return None;
        }

        let slice = &line[char_index..char_index + match_word.len()];
        if slice == match_word {
            Some(MatchLocation {
                start_line: line_index,
                start_char: char_index,
                direction: "forward",
            })
        } else {
            None
        }
    }

    pub fn backward(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        let line = rows[line_index];
        if char_index + 1 < match_word.len() {
            return None;
        }

        let start = char_index + 1 - match_word.len();
        let slice = &line[start..=char_index];
        let reversed_match = match_word.chars().rev().collect::<String>();

        if slice == reversed_match {
            Some(MatchLocation {
                start_line: line_index,
                start_char: char_index,
                direction: "backward",
            })
        } else {
            None
        }
    }

    pub fn upwards(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        if line_index + 1 < match_word.len() {
            return None;
        }

        for (offset, char) in match_word.chars().enumerate() {
            if char_index >= rows[line_index - offset].len() ||
                rows[line_index - offset].chars().nth(char_index) != Some(char) {
                return None;
            }
        }

        Some(MatchLocation {
            start_line: line_index,
            start_char: char_index,
            direction: "upwards",
        })
    }

    pub fn downwards(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        if line_index + match_word.len() > rows.len() {
            return None;
        }

        for (offset, char) in match_word.chars().enumerate() {
            if char_index >= rows[line_index + offset].len() ||
                rows[line_index + offset].chars().nth(char_index) != Some(char) {
                return None;
            }
        }

        Some(MatchLocation {
            start_line: line_index,
            start_char: char_index,
            direction: "downwards",
        })
    }

    pub fn down_right(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        if line_index + match_word.len() > rows.len() ||
            char_index + match_word.len() > rows[line_index].len() {
            return None;
        }

        for (offset, char) in match_word.chars().enumerate() {
            if char_index + offset >= rows[line_index + offset].len() ||
                rows[line_index + offset].chars().nth(char_index + offset) != Some(char) {
                return None;
            }
        }

        Some(MatchLocation {
            start_line: line_index,
            start_char: char_index,
            direction: "down_right",
        })
    }

    pub fn down_left(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        if line_index + match_word.len() > rows.len() ||
            char_index + 1 < match_word.len() {
            return None;
        }

        for (offset, char) in match_word.chars().enumerate() {
            if char_index < offset ||
                rows[line_index + offset].chars().nth(char_index - offset) != Some(char) {
                return None;
            }
        }

        Some(MatchLocation {
            start_line: line_index,
            start_char: char_index,
            direction: "down_left",
        })
    }

    pub fn up_right(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        if line_index + 1 < match_word.len() ||
            char_index + match_word.len() > rows[line_index].len() {
            return None;
        }

        for (offset, char) in match_word.chars().enumerate() {
            if char_index + offset >= rows[line_index - offset].len() ||
                rows[line_index - offset].chars().nth(char_index + offset) != Some(char) {
                return None;
            }
        }

        Some(MatchLocation {
            start_line: line_index,
            start_char: char_index,
            direction: "up_right",
        })
    }

    pub fn up_left(rows: &[&str], line_index: usize, char_index: usize, match_word: &str) -> Option<MatchLocation> {
        if line_index + 1 < match_word.len() ||
            char_index + 1 < match_word.len() {
            return None;
        }

        for (offset, char) in match_word.chars().enumerate() {
            if char_index < offset ||
                rows[line_index - offset].chars().nth(char_index - offset) != Some(char) {
                return None;
            }
        }

        Some(MatchLocation {
            start_line: line_index,
            start_char: char_index,
            direction: "up_left",
        })
    }

    pub fn x_pattern_mas(rows: &[&str], line_index: usize, char_index: usize, _match_word: &str) -> Option<MatchLocation> {
        // Check if we have enough room to search diagonally
        if line_index == 0 || line_index >= rows.len() - 1 ||
            char_index == 0 || char_index >= rows[line_index].len() - 1 {
            return None;
        }

        // Check if current position is 'A'
        if rows[line_index].chars().nth(char_index) != Some('A') {
            return None;
        }

        // Get all diagonal positions
        let up_left = rows[line_index - 1].chars().nth(char_index - 1);
        let up_right = rows[line_index - 1].chars().nth(char_index + 1);
        let down_left = rows[line_index + 1].chars().nth(char_index - 1);
        let down_right = rows[line_index + 1].chars().nth(char_index + 1);

        // Check both diagonals - BOTH must spell "MAS"
        let diagonal1_valid = (up_left == Some('M') && down_right == Some('S')) ||
            (up_left == Some('S') && down_right == Some('M'));

        let diagonal2_valid = (up_right == Some('M') && down_left == Some('S')) ||
            (up_right == Some('S') && down_left == Some('M'));

        // Only return a match if both diagonals spell "MAS"
        if diagonal1_valid && diagonal2_valid {
            Some(MatchLocation {
                start_line: line_index,
                start_char: char_index,
                direction: "x_pattern_mas",
            })
        } else {
            None
        }
    }

    // Implement Display trait for MatchLocation (optional, but helpful for debugging)
    impl fmt::Display for MatchLocation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Match at line {}, char {}, direction: {}",
                   self.start_line, self.start_char, self.direction)
        }
    }
}