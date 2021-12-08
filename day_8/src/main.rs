use core::num;
use std::{fs, iter::Map, collections::HashMap, result::IterMut};
use itertools::*;

fn main() {
    let arr = parse_file("./input.txt");
}

fn parse_file(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|it| {
            let line: (Vec<String>, Vec<String>) = it
            .split("|")
            .map(|it| it.split(" ").map(|it| it.chars().sorted().collect::<String>()).collect())
            .collect_tuple()
            .unwrap();
            line
        })
        .collect::<Vec<(Vec<String>, Vec<String>)>>()
}

fn determine_numbers_from_line(line: &Vec<String>) -> HashMap<String, i32> {
    let mut result = HashMap::new();
    let num_8 = line.iter().find(|it| it.len() == 7);
    match num_8 {
        Some(it) => {result.insert(it.to_owned(), 8);},
        None => {},
    };
    

    let num_7 = line.iter().find(|it| it.len() == 3);
    match num_7 {
        Some(it) => {result.insert(it.to_owned(), 7);},
        None => {},
    };

    let num_1 = line.iter().find(|it| it.len() == 2);
    match num_1 {
        Some(it) => {result.insert(it.to_owned(), 1);},
        None => {},
    };

    let num_4 = line.iter().find(|it| it.len() == 4);
    match num_4 {
        Some(it) => {result.insert(it.to_owned(), 4);},
        None => {},
    };


    return result;
}

fn line_to_numbers(line: &Vec<String>) -> Vec<i32> {
    let line_to_num = determine_numbers_from_line(line);
    line.iter().map(|it| {
        match line_to_num.get(it) {
            Some(it) => it.to_owned(),
            None => i32::MAX
        }
    }).collect()

}

fn line_tuple_to_numbers(line: &(Vec<String>, Vec<String>)) -> (Vec<i32>, Vec<i32>) {
    (line_to_numbers(&line.0), line_to_numbers(&line.1))
}


fn calculate_part_one(arr: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
    arr.iter()
    .map(|it| line_tuple_to_numbers(it))
    .map(|it| it.1)
    .flatten()
    .fold(0, |acc, num| {
        if num == 1 || num == 4 || num == 7 || num == 8 {
            return acc+1;
        } else {
            return acc;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_line_tuple_to_numbers() {
        let arr = parse_file("./exampleInput.txt");
        let line_tuple = line_tuple_to_numbers(&arr[0]);
        assert_eq!(line_tuple.0, [1, 8, 2147483647, 2147483647, 4, 2147483647, 2147483647, 2147483647, 2147483647, 7, 2147483647]);
    }

    #[test]
    fn test_calculate_part_one_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let test_result = calculate_part_one(&arr);
        assert_eq!(test_result, 26);
    }

    #[test]
    fn test_calculate_part_one_file() {
        let arr = parse_file("./input.txt");
        let test_result = calculate_part_one(&arr);
        assert_eq!(test_result, 321);
    }

}
