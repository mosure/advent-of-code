use std::collections::HashSet;


fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn sum_part_numbers(schematic: &str) -> i32 {
    let grid: Vec<Vec<char>> = schematic.lines().map(|line| line.chars().collect()).collect();

    let mut total_sum = 0;

    for (i, row) in grid.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if row[j].is_digit(10) {
                if j == 0 || !row[j - 1].is_digit(10) {
                    let number: String = row[j..].iter().take_while(|&&c| c.is_digit(10)).collect();
                    j += number.len();
                    let number: i32 = number.parse().unwrap();
                    if (0..number.to_string().len()).any(|k| {
                        (-1..=1).any(|dx| (-1..=1).any(|dy| {
                            let (nx, ny) = (i as i32 + dx, (j - k) as i32 - 1 + dy);
                            nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < row.len() as i32 && is_symbol(grid[nx as usize][ny as usize])
                        }))
                    }) {
                        total_sum += number;
                    }
                }
            }
            j += 1;
        }
    }

    total_sum
}

fn calculate_gear_ratios(schematic: &str) -> i32 {
    let grid: Vec<Vec<char>> = schematic.lines().map(|line| line.chars().collect()).collect();

    fn extract_full_number(grid: &[Vec<char>], mut x: usize, mut y: usize) -> Option<i32> {
        let orig_y = y;
        if y > 0 && grid[x][y - 1].is_digit(10) {
            while y > 0 && grid[x][y - 1].is_digit(10) {
                y -= 1;
            }
        }
        let mut number: String = grid[x][y..].iter().take_while(|&&c| c.is_digit(10)).collect();
        if !number.is_empty() {
            return number.parse().ok();
        }

        y = orig_y;
        if x > 0 && grid[x - 1][y].is_digit(10) {
            while x > 0 && grid[x - 1][y].is_digit(10) {
                x -= 1;
            }
        }
        number = grid.iter().skip(x).map(|row| row[y]).take_while(|&c| c.is_digit(10)).collect();
        number.parse().ok()
    }

    fn find_adjacent_parts(grid: &[Vec<char>], x: usize, y: usize) -> Vec<i32> {
        let mut parts = HashSet::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid[0].len() as i32 && grid[nx as usize][ny as usize].is_digit(10) {
                    if let Some(part_number) = extract_full_number(grid, nx as usize, ny as usize) {
                        parts.insert(part_number);
                    }
                }
            }
        }
        parts.into_iter().collect()
    }

    let mut total_gear_ratio = 0;
    let mut visited_gears = HashSet::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell == '*' && visited_gears.insert((x, y)) {
                let adjacent_parts = find_adjacent_parts(&grid, x, y);
                if adjacent_parts.len() == 2 {
                    total_gear_ratio += adjacent_parts[0] * adjacent_parts[1];
                }
            }
        }
    }

    total_gear_ratio
}

fn main() {
    let schematic = include_str!("3.input");

    println!("{}", sum_part_numbers(schematic));
    println!("{}", calculate_gear_ratios(schematic));
}
