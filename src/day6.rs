use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_movement(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct GuardState {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    initial_guard: GuardState,
    width: usize,
    height: usize,
}

impl Map {
    fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut grid = Vec::new();
        let mut guard = None;

        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            let mut row = Vec::new();

            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '^' => {
                        guard = Some(GuardState {
                            x,
                            y,
                            direction: Direction::Up,
                        });
                        row.push('.');
                    },
                    '.' | '#' => row.push(ch),
                    _ => return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid character in map: {}", ch)
                    )),
                }
            }
            grid.push(row);
        }

        let height = grid.len();
        let width = grid[0].len();
        let initial_guard = guard.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "No guard found in map")
        })?;

        Ok(Map {
            grid,
            initial_guard,
            width,
            height,
        })
    }

    fn is_obstruction(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 ||
            x >= self.width as isize ||
            y >= self.height as isize {
            return true;
        }
        self.grid[y as usize][x as usize] == '#'
    }

    fn simulate_with_extra_obstruction(&self, obs_x: usize, obs_y: usize) -> bool {
        // Don't place obstruction at guard's starting position
        if obs_x == self.initial_guard.x && obs_y == self.initial_guard.y {
            return false;
        }

        // Create modified grid with extra obstruction
        let mut modified_grid = self.grid.clone();
        modified_grid[obs_y][obs_x] = '#';

        let mut guard = self.initial_guard;
        let mut visited_states = HashSet::new();

        loop {
            // Store current state
            if !visited_states.insert(guard) {
                // Found a loop!
                return true;
            }

            // Check what's in front
            let (dx, dy) = guard.direction.get_movement();
            let next_x = guard.x as isize + dx;
            let next_y = guard.y as isize + dy;

            // Check if guard would leave map
            if next_x < 0 || next_y < 0 ||
                next_x >= self.width as isize ||
                next_y >= self.height as isize {
                return false;  // Guard leaves map, no loop
            }

            // Check for obstruction (including new one)
            if next_x >= 0 && next_y >= 0 &&
                (modified_grid[next_y as usize][next_x as usize] == '#') {
                guard.direction = guard.direction.turn_right();
            } else {
                guard.x = next_x as usize;
                guard.y = next_y as usize;
            }
        }
    }

    fn count_possible_loop_obstructions(&self) -> usize {
        let mut count = 0;
        let mut loop_positions = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                // Skip if already an obstruction
                if self.grid[y][x] == '#' {
                    continue;
                }
                // Try adding obstruction here
                if self.simulate_with_extra_obstruction(x, y) {
                    count += 1;
                    loop_positions.push((x, y));
                }
            }
        }

        // Print the positions that create loops
        println!("\nPositions that create loops:");
        for (x, y) in loop_positions {
            println!("Position ({}, {})", x, y);
        }

        count
    }
}

pub fn process(input_file_path: &str) -> io::Result<()> {
    let mut map = Map::from_file(input_file_path)?;

    println!("Analyzing possible obstruction positions...");

    let loop_count = map.count_possible_loop_obstructions();
    println!("\nFound {} positions where adding an obstruction creates a loop", loop_count);

    Ok(())
}