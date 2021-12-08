use itertools::*;
use std::{collections::HashMap, fs};

fn main() {
    let arr = parse_file("./input.txt");
    let test_result = calculate_part_one(&arr);
    println!("Result from part 1: {}", test_result);

    let test_result_2 = calculate_part_two(&arr);
    println!("Result from part 2: {}", test_result_2);
}

fn parse_file(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|it| {
            let line: (Vec<String>, Vec<String>) = it
                .split(" | ")
                .map(|it| {
                    it.split(" ")
                        .map(|it| it.chars().sorted().collect::<String>())
                        .collect()
                })
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
        Some(it) => {
            result.insert(it.to_owned(), 8);
            println!("found 8");
        }
        None => {}
    };

    let num_7 = line.iter().find(|it| it.len() == 3);
    match num_7 {
        Some(it) => {
            println!("found 7");
            result.insert(it.to_owned(), 7);

            let num_3 = line.iter().find(|it| {
                it.len() == 5
                    && it
                        .chars()
                        .filter(|it| !num_7.unwrap().to_owned().contains(it.to_owned()))
                        .count()
                        == 2
            });

            match num_3 {
                Some(it) => {
                    result.insert(it.to_owned(), 3);
                    println!("found 3");
                    println!("it was {}", it);
                }
                None => {}
            };

            let num_6 = line.iter().find(|it| {
                it.len() == 6
                    && it
                        .chars()
                        .filter(|it| !num_7.unwrap().to_owned().contains(it.to_owned()))
                        .count()
                        == 4
            });

            match num_6 {
                Some(it) => {
                    result.insert(it.to_owned(), 6);
                    println!("found 6");
                    let num_5 = line.iter().find(|it| {
                        it.len() == 5
                            && it
                                .chars()
                                .filter(|it| !num_6.unwrap().to_owned().contains(it.to_owned()))
                                .count()
                                == 0
                    });

                    match num_5 {
                        Some(it) => {
                            result.insert(it.to_owned(), 5);
                            println!("found 5");
                        }
                        None => {}
                    };
                }
                None => {}
            };
        }
        None => {}
    };

    let num_1 = line.iter().find(|it| it.len() == 2);
    match num_1 {
        Some(it) => {
            result.insert(it.to_owned(), 1);
            println!("found 1");
        }
        None => {}
    };

    let num_4 = line.iter().find(|it| it.len() == 4);
    match num_4 {
        Some(it) => {
            println!("found 4");
            result.insert(it.to_owned(), 4);

            let num_9 = line.iter().find(|it| {
                it.len() == 6
                    && it
                        .chars()
                        .filter(|it| !num_4.unwrap().to_owned().contains(it.to_owned()))
                        .count()
                        == 2
            });

            match num_9 {
                Some(it) => {
                    result.insert(it.to_owned(), 9);
                    println!("found 9");

                    let num_2 = line.iter().find(|it| {
                        it.len() == 5
                            && it
                                .chars()
                                .filter(|it| !num_9.unwrap().to_owned().contains(it.to_owned()))
                                .count()
                                == 1
                    });

                    match num_2 {
                        Some(it) => {
                            result.insert(it.to_owned(), 2);
                            println!("found 2");
                        }
                        None => {}
                    };
                }
                None => {}
            };
        }
        None => {}
    };

    line.iter().filter(|it| it.len() == 6).for_each(|it| {
        if !result.contains_key(it) {
            println!("found 0");
            println!("it was {}", it);
            result.insert(it.to_owned(), 0);
        }
    });

    return result;
}

fn line_tuple_to_numbers(line: &(Vec<String>, Vec<String>)) -> (Vec<i32>, Vec<i32>) {
    let line_to_num = determine_numbers_from_line(&line.0);
    (
        line.0
            .iter()
            .map(|it| match line_to_num.get(it) {
                Some(it) => it.to_owned(),
                None => {
                    println!("Could not find {}", it);
                    i32::MAX
                }
            })
            .collect(),
        line.1
            .iter()
            .map(|it| match line_to_num.get(it) {
                Some(it) => it.to_owned(),
                None => {
                    println!("Could not find {}", it);
                    i32::MAX
                }
            })
            .collect(),
    )
}

fn numbers_to_num(numbers: Vec<i32>) -> i32 {
    numbers.iter().join("").parse::<i32>().unwrap()
}

fn calculate_part_two(arr: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
    arr.iter()
        .map(|it| line_tuple_to_numbers(it))
        .map(|it| numbers_to_num(it.1))
        .fold(0, |acc, num| {
            return acc + num;
        })
}

fn calculate_part_one(arr: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
    arr.iter()
        .map(|it| line_tuple_to_numbers(it))
        .map(|it| it.1)
        .flatten()
        .fold(0, |acc, num| {
            if num == 1 || num == 4 || num == 7 || num == 8 {
                return acc + 1;
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
        assert_eq!(line_tuple.0, [1, 8, 9, 6, 4, 5, 0, 3, 2, 7]);
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

    #[test]
    fn find_9_from_4() {
        let num_9 = "cefabd"
            .chars()
            .filter(|it| !"eafb".to_owned().contains(it.to_owned()))
            .count();
        assert_eq!(num_9, 2);
    }

    #[test]
    fn test_calculate_part_two_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let test_result = calculate_part_two(&arr);
        assert_eq!(test_result, 61229);
    }
    
    #[test]
    fn test_calculate_part_two_file() {
        let arr = parse_file("./input.txt");
        let test_result = calculate_part_two(&arr);
        assert_eq!(test_result, 1028926);
    }
}
