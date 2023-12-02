#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn add(&mut self, color: &str, count: u32) {
        match color {
            "red" => self.red += count,
            "green" => self.green += count,
            "blue" => self.blue += count,
            _ => panic!("Invalid color"),
        }
    }
}

struct CubeLimits {
    red: u32,
    green: u32,
    blue: u32,
}


fn parse_game_data(game_line: &str) -> (u32, Vec<Cubes>) {
    let (game_id_str, draws_str) = game_line.split_once(':').unwrap();
    let game_id = game_id_str.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();

    let game_data = draws_str.split(';')
        .map(parse_draw)
        .collect();

    (game_id, game_data)
}

fn parse_draw(draw: &str) -> Cubes {
    draw.trim().split(',').fold(
        Cubes::default(),
        |mut acc, color_info| {
            let parts = color_info.trim().split_whitespace().collect::<Vec<&str>>();
            let count = parts[0].parse::<u32>().unwrap();
            let color = parts[1];
            acc.add(color, count);
            acc
        }
    )
}

fn is_game_possible(game_data: &[Cubes], limits: &CubeLimits) -> bool {
    game_data.iter().all(|cubes| {
        cubes.red <= limits.red && cubes.green <= limits.green && cubes.blue <= limits.blue
    })
}

fn sum_of_possible_games(game_lines: &[&str], limits: &CubeLimits) -> u32 {
    game_lines.iter()
        .map(|&line| parse_game_data(line))
        .filter(|(_, game_data)| is_game_possible(game_data, limits))
        .map(|(game_id, _)| game_id)
        .sum()
}

fn minimum_cubes_for_game(game_data: &[Cubes]) -> (u32, u32, u32) {
    game_data.iter().fold(
        (0, 0, 0),
        |(min_red, min_green, min_blue), cubes| {
            let max_red = std::cmp::max(min_red, cubes.red);
            let max_green = std::cmp::max(min_green, cubes.green);
            let max_blue = std::cmp::max(min_blue, cubes.blue);
            (max_red, max_green, max_blue)
        }
    )
}

fn sum_of_power_of_minimum_sets(game_lines: &[&str]) -> u32 {
    game_lines.iter()
        .map(|&line| parse_game_data(line))
        .map(|(_, game_data)| minimum_cubes_for_game(&game_data))
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}


fn main() {
    let input = include_str!("2.input");

    let game_lines: Vec<&str> = input.lines().collect();

    let p1 = sum_of_possible_games(
        &game_lines,
        &CubeLimits {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    let p2 = sum_of_power_of_minimum_sets(&game_lines);

    println!("p1: {}", p1);
    println!("p2: {}", p2);
}
