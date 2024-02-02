use std::fs;

fn vec_to_num(vector: Vec<u32>) -> u32 {
    let mut num = 0;
    //if vector.len() > 1 {
        num = vector[0]*10 + vector.last().unwrap();
    //} else {
    //    num = vector[0];
        //}
    println!("{}", num);
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

fn plain_letters_to_numbers(line: &str) -> String {
    let mut new_line;
    new_line = line.replace("two", "t2o");
    new_line = new_line.replace("nine", "n9e");
    new_line = new_line.replace("three", "t3e");
    new_line = new_line.replace("five", "f5e");
    new_line = new_line.replace("eight", "e8t");
    new_line = new_line.replace("one","o1e");
    new_line = new_line.replace("six", "s6x");
    new_line = new_line.replace("seven", "s7n");
    new_line = new_line.replace("four", "f4r");
    new_line = new_line.replace("zero", "z0o");
    new_line

}
fn find_number(line: &str) -> Vec<u32> {
        let mut vec_num = Vec::new();
        let digit_line = plain_letters_to_numbers(line);
        println!("{}", line);
        for character in digit_line.chars() {
            if character.is_numeric() {
                vec_num.push(character.to_digit(10).expect("not a number"));
            }
        }
        vec_num
}

fn main() {
    let mut sum = 0;
    let (ret, total) = readline("/Users/clem/Projets/prog/advent2023/day1.input");
    for x in 0..total {
        sum += vec_to_num(find_number(&ret[x].clone()));
    }
    println!("{}",sum);
}
