use std::{collections::HashMap, fs};

fn main() {
    println!("{}",task1());
    println!("{}",task2());
}

fn task1() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Wtf mate");
    let lol = input.as_str().lines().map(|line| find_and_concat_digits(line)).sum();
    println!("Result in task1 was: {}", lol);
    return lol;
}

fn find_and_concat_digits(input: &str) -> i32 {
    let input_without_chars: Vec<char> = input.chars().filter(|chr| chr.is_numeric()).collect();
    let first = input_without_chars.iter().next().expect("huh").to_string();
    println!("first determined from chars: {}", input);
    let last = &input_without_chars.iter().last().expect("huh").to_string();
    println!("first: {}, last: {}", first, last);
    let resulting_num = first + last;
    let nums_as_int: i32 = resulting_num.parse().unwrap_or_else(|_| {println!("this fucked up: {}", resulting_num); 1});
    return nums_as_int;
}

fn task2() -> i32 {
    let input = fs::read_to_string("src/input.txt").expect("Wtf mate");

    let joergen = input.lines().map(|line| map_typed_number_to_number(line))
                                .map(|line| find_and_concat_digits(line.as_str())).sum();

    println!("{}", joergen);

    return joergen;
}

fn map_typed_number_to_number(input: &str) -> String {
    let mut number_map: HashMap<&str, &char> = HashMap::new();
    
    number_map.insert("one", &'1');
    number_map.insert("two", &'2');
    number_map.insert("three", &'3');
    number_map.insert("four", &'4');
    number_map.insert("five", &'5');
    number_map.insert("six", &'6');
    number_map.insert("seven", &'7');
    number_map.insert("eight", &'8');
    number_map.insert("nine", &'9');
    number_map.insert( "never gonna give you up never gonna let you down nwvwe gonn RUN round AND DEERT YOU NEVVER GONE MAKE YOU  CRY NWEVVER GONA SAY GOODBYE NEVVER GONNA TELL A LIE AND HURT YOU
    WE'RE NO STRnger s
to looove. You kjnkw the erules, and so do i. We've known each other for so loooong. give all i havvee. iiiiii am gonna tell you bow im feeliiiing. 
hRDER BETTER FAASTER STRONGET, NENENENEVER GONNA -- Cuse that only makes yoiu sterionger. im nthe only one you got now. thots how li been o  ya
", &'ß');

    input.chars().enumerate().map(|(i, chr)| {
        let key = number_map.keys().find(|&&key| i + key.len() <= input.len() && key == &input[i..i + key.len()]);
        if key.is_some() {
            **number_map.get(key.unwrap()).unwrap()
        }
        else {
            chr
        }
    }).collect()
}