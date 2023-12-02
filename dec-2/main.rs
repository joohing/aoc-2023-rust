use std::fs;
use regex::Regex;

fn main() {
    println!("Task 1 gave output: {}", task1());
    println!("Task 2 gave output: {}", task2());
}

// 12 red, 13 green, 14 blue: sum of IDs of possible games?
fn task1() -> u32 {
    let input: String = fs::read_to_string("src/input.txt").unwrap();
    let input_lines: Vec<&str> = input.split("\n").collect();
    
    let valid_games_tuples: Vec<(u32, i32, i32, i32)> = input_lines.iter()
                                    .map(|line| filter_one_line(line))
                                    .filter(|res_tuple| res_tuple.1 <= 12 && res_tuple.2 <= 13 && res_tuple.3 <= 14)
                                    .collect();

    valid_games_tuples.iter().map(|tuple| tuple.0).sum::<u32>()
}

// Minimum amt of cubes for each game multiplied together, and then summed?
fn task2() -> u32 {
    let input: String = fs::read_to_string("src/input.txt").unwrap();
    let input_lines: Vec<&str> = input.split("\n").collect();
    
    let min_cubes: Vec<(u32, i32, i32, i32)> = input_lines.iter()
                                    .map(|line| filter_one_line(line))
                                    .collect();

    let powers: Vec<u32> = min_cubes.iter().map(move |&tuple| (tuple.1 * tuple.2 * tuple.3) as u32).collect();

    powers.iter().sum()
}

// return game ID, and max number of cubes for each colour that occurred on that line
fn filter_one_line(line: &str) -> (u32, i32, i32, i32) {
    let game_id_pattern = Regex::new(r"Game ([0-9]+)").unwrap();
    let (red_pat, green_pat, blue_pat) = (Regex::new(r" (..?) red").unwrap(), 
                                        Regex::new(r" (..?) green").unwrap(), 
                                        Regex::new(r" (..?) blue").unwrap());

    let game_id = game_id_pattern.captures(line).unwrap().get(1).unwrap();
    let (red_cap, green_cap, blue_cap) = (red_pat.captures_iter(line),
    green_pat.captures_iter(line),
    blue_pat.captures_iter(line));
    
    let (n_red, n_green, n_blue) = (find_max_of_capture(red_cap), find_max_of_capture(green_cap), find_max_of_capture(blue_cap));
    let game_id_int = game_id.as_str().parse().unwrap();

    return (game_id_int, n_red, n_green, n_blue);
}

fn find_max_of_capture(capture_list: regex::CaptureMatches<'_, '_>) -> i32 {
    let nums_for_cube_colour = capture_list.map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<&str>>();
    nums_for_cube_colour.iter().map(|&chr| chr.parse().unwrap()).max().unwrap()
}