use std::fs;

fn vec_to_num(vector: Vec<u32>) -> u32 {
    let num = vector[0]*10 + vector.last().unwrap();
    num
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

fn find_number(line: &str) -> Vec<u32> {
        let mut vec_num = Vec::new();
        for character in line.chars() {
            if character.is_numeric() {
                vec_num.push(character.to_digit(10).expect("not a number"));
            }
        }
        vec_num
}

fn main() {
    let mut sum = 0;
    let (ret, total) = readline("/home/clem/Projets/adventofcode/day1.input");
    for x in 0..total {
        sum += vec_to_num(find_number(&ret[x].clone()));
    }
    println!("{}",sum);
}
