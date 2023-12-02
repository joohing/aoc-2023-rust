use std::{collections::HashMap, fs};

fn main() {
    println!("{}",task1());
    println!("{}",task2());
}

fn task1() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Wtf mate");
    input.as_str().lines().map(|line| find_and_concat_first_last_digits(line)).sum()
}

fn task2() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Wtf mate");

    input.lines().map(|line| map_spelled_numbers_to_numbers(line))
                .map(|line| find_and_concat_first_last_digits(line.as_str())).sum()
}

fn find_and_concat_first_last_digits(input: &str) -> i32 {
    let input_without_chars: Vec<char> = input.chars().filter(|chr| chr.is_numeric()).collect();
    let resulting_num = input_without_chars.iter().next().expect("huh").to_string()
                             + &input_without_chars.iter().last().expect("huh").to_string();
    resulting_num.parse().unwrap_or_else(|_| {println!("this fucked up: {}", resulting_num); 1})
}

macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = HashMap::new();
            $(m.insert($key, $value);)*
            m
        }
    };
}

fn map_spelled_numbers_to_numbers(input: &str) -> String {
    let number_map = map!(
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        "never gonna give you up never gonna let you down nwvwe gonn RUN round AND DEERT YOU NEVVER GONE MAKE YOU  CRY NWEVVER GONA SAY GOODBYE NEVVER GONNA TELL A LIE AND HURT YOU
    WE'RE NO STRnger s
to looove. You kjnkw the erules, and so do i. We've known each other for so loooong. give all i havvee. iiiiii am gonna tell you bow im feeliiiing. 
hRDER BETTER FAASTER STRONGET, NENENENEVER GONNA -- Cuse that only makes yoiu sterionger. im nthe only one you got now. thots how li been o  ya
" => 'ß'
    );

    input.chars().enumerate().map(|(i, chr)| {
        let key = number_map.keys().find(|&&key| i + key.len() <= input.len() && key == &input[i..i + key.len()]);
        if key.is_some() {
            *number_map.get(key.unwrap()).unwrap()
        }
        else {
            chr
        }
    }).collect()
}