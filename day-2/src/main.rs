use std::fs;
use regex::Regex;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

const RED_REG: &str = "[0-9]+ red";
const BLUE_REG: &str = "[0-9]+ blue";
const GREEN_REG: &str = "[0-9]+ green";
const NUM_REG: &str = "[0-9]+";


fn is_possible(red: i32, green: i32, blue: i32) -> i32 {
    if red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE {
       0
    } else {
       1
    }
}

fn analyze_set(set: &str) -> (i32, i32, i32) {
// one set must be used to fill a GameSet struct
    let mut rgb: (i32, i32, i32) = (0, 0, 0);
    let set_iter = set.split(",");
    let num_regex = Regex::new(NUM_REG).unwrap();
    let red_regex = Regex::new(RED_REG).unwrap();
    let green_regex = Regex::new(GREEN_REG).unwrap();
    let blue_regex = Regex::new(BLUE_REG).unwrap();
    println!("{}", set);
    for cubes in set_iter {
        if red_regex.is_match(cubes) {
            rgb.0 = num_regex.captures(cubes)
                                    .expect("Not a number")
                                    .get(0)
                                    .unwrap()
                                    .as_str()
                                    .parse()
                                    .unwrap();
        } else if green_regex.is_match(cubes) {
            rgb.1 = num_regex.captures(cubes)
                                    .expect("Not a number")
                                    .get(0)
                                    .unwrap()
                                    .as_str()
                                    .parse()
                                    .unwrap();
        } else if blue_regex.is_match(cubes) {
            rgb.2 = num_regex.captures(cubes)
                                    .expect("Not a number")
                                    .get(0)
                                    .unwrap()
                                    .as_str()
                                    .parse()
                                    .unwrap();
 
        } else {
            println!("NOT POSSIBLE");
        }
    }
    rgb
}

fn get_sets_from_game(game: &str, game_num: usize) -> i32 {
   let sets = game.split(";");
   let mut ret: (i32, i32, i32) = (0, 0, 0);
   let mut min_red = 0; 
   let mut min_green = 0; 
   let mut min_blue = 0;

   println!("game {}", game_num);
    for set in sets {
        // return 0 if set is possible 1 else. then return the return
        ret = analyze_set(set);
    
        if ret.0 > min_red {
            min_red = ret.0;
        }
        if ret.1 > min_green {
            min_green = ret.1;
        }
        if ret.2 > min_blue {
            min_blue = ret.2;
        }
    }
    if is_possible(ret.0, ret.1, ret.2) == 0 {
        println!("POSSIBLE");
    } else {
        println!("PAS POSSIBLE");
    }
    println!("min for game is red {} green {} blue {}", min_red, min_green, min_blue);
    min_red * min_green * min_blue
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn readline(path: &str) -> (Vec<String>, usize) {
    let content = read_lines(path);
    let len = content.len();
    (content, len)
}

fn discard_line_begining(line: &str) -> &str {
    let mut temp = line.split(":");
    temp.next(); // blank to get what's after "Game X :"
    temp.next().expect("Not a string")
}

fn main() {
    let mut sum = 0;
    let (ret, _total) = readline("/Users/clem/Projets/prog/advent2023/day-2/src/input.txt");
    for (idx, x) in ret.iter().enumerate() {
        sum += get_sets_from_game(discard_line_begining(&*x), idx+1);
    }
    println!("{}", sum);
}
