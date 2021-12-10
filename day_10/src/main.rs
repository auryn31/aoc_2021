use itertools::*;
use std::{collections::HashMap, fs, result::IterMut};

fn main() {
    let arr = parse_file("./input.txt");
    // let test_result = calculate_part_one(&arr);
    // println!("Result from part 1: {}", test_result);

    // let test_result_2 = calculate_part_two(&arr);
    // println!("Result from part 2: {}", test_result_2);
}

fn parse_file(filename: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|it| {
            it.split("")
                .filter(|it| it.len() == 1)
                .map(|it| it.to_string())
                .collect()
        })
        .collect::<Vec<Vec<String>>>()
}

fn find_wrong_bracket(line: &Vec<String>) -> Option<String> {
    let mut stack = Vec::new();
    let mut wrong_bracket: Option<String> = None;
    for bracket in line {
        match bracket.as_ref() {
            "(" => stack.push(")"),
            "{" => stack.push("}"),
            "[" => stack.push("]"),
            "<" => stack.push(">"),
            it => {
                if bracket != stack.pop().unwrap() {
                    wrong_bracket = Some(bracket.to_owned());
                    break;
                }
            }
        }
    }
    return wrong_bracket;
}

fn fill_line(line: &Vec<String>) -> Vec<String> {
    let mut stack = Vec::new();
    let mut wrong_bracket: Option<String> = None;
    for bracket in line {
        match bracket.as_ref() {
            "(" => stack.push(")".to_owned()),
            "{" => stack.push("}".to_owned()),
            "[" => stack.push("]".to_owned()),
            "<" => stack.push(">".to_owned()),
            it => {
                if bracket.to_owned() != stack.pop().unwrap() {
                    wrong_bracket = Some(bracket.to_owned());
                    break;
                }
            }
        }
    }
    if wrong_bracket.is_some() {
        panic!("Shish")
    }
    stack.reverse();
    return stack;
}

fn calculate_part_one_sum(caracter: String) -> i128 {
    return match caracter.as_ref() {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        it => panic!(format!("Didnt expect {}", it))
    };
}

fn map_bracket_to_num_part_two(caracter: String) -> i128 {
    return match caracter.as_ref() {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4,
        it => panic!(format!("Didnt expect {}", it))
    };
}

fn calculate_sum_for_line(line: Vec<i128>) -> i128 {
    let mut sum = 0;
    for num in line {
        sum = sum * 5 + num;
    }
    return sum;
}

fn find_median(lines: Vec<i128>) -> i128 {
    let mut sorted_lines = lines.to_vec();
    sorted_lines.sort();
    return sorted_lines[(sorted_lines.len()-1)/2];
}

fn calculate_part_two(lines: &Vec<Vec<String>>) -> i128 {
    let numbers = lines.iter()
    .filter(|it| find_wrong_bracket(it).is_none())
    .map(|it| {
        let numbers_line: Vec<i128> = fill_line(it).iter().map(|it| map_bracket_to_num_part_two(it.to_owned())).collect();
        return calculate_sum_for_line(numbers_line);
    })
    .collect();
    return find_median(numbers);
}

fn calculate_part_one(lines: &Vec<Vec<String>>) -> i128 {
    lines.iter()
    .map(|it| find_wrong_bracket(it))
    .filter(|it| it.is_some())
    .map(|it| calculate_part_one_sum(it.unwrap()))
    .fold(0, |acc, sum| acc+sum)
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
    fn test_wrong_bracket_line_one_test_file_line_2() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(find_wrong_bracket(&arr[2]).unwrap(), "}");
    }

    #[test]
    fn test_wrong_bracket_line_one_test_file_line_0() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(find_wrong_bracket(&arr[0]), None);
    }
    
    #[test]
    fn test_fill_brackets_one_test_file_line_0() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(fill_line(&arr[0]).join(""), "}}]])})]");
    }

    #[test]
    fn test_calculate_part_one_example() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(calculate_part_one(&arr), 26397);
    }

    #[test]
    fn test_calculate_part_two_example() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(calculate_part_two(&arr), 288957);
    }

    #[test]
    fn test_calculate_part_one() {
        let arr = parse_file("./input.txt");
        assert_eq!(calculate_part_one(&arr), 413733);
    }

    #[test]
    fn test_calculate_part_two() {
        let arr = parse_file("./input.txt");
        assert_eq!(calculate_part_two(&arr), 3354640192);
    }


    #[test]
    fn test_stack() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop().unwrap(), 3);
    }
}
