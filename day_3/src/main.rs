use std::fs;

use itertools::*;
fn main() {
    let filename = "./input.txt";
    run_for_file(filename);
}

fn run_for_file(filename: &str) {
    println!("In file {}", filename);
    let arr = parse_file(filename);
    let position = part_1(&arr);
    println!("part 1 {}", position);
    let position_2 = part_2(&arr);
    println!("part 2 {}", position_2);
}

fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents.split("\n").map(|it| transform(it)).collect();
}

fn part_1(arr: &Vec<Vec<i32>>) -> i32 {
    let arr_len = arr.len() as i32;
    let result = arr
        .iter()
        .fold(vec![0; arr[0].len()], |acc, line| {
            acc.iter()
                .zip(line.iter())
                .map(|(a, b)| a + b)
                .collect_vec()
        })
        .iter()
        .map(|line| {
            (
                if line > &(arr_len - line) { 1 } else { 0 },
                if line < &(arr_len - line) { 1 } else { 0 },
            )
        })
        .fold(("".to_string(), "".to_string()), |acc, line| {
            (
                format!("{}{}", acc.0, line.0),
                format!("{}{}", acc.1, line.1),
            )
        });

    let gamma_rate = isize::from_str_radix(&result.0, 2).unwrap() as i32;
    let epsilon_rate = isize::from_str_radix(&result.1, 2).unwrap() as i32;

    return gamma_rate * epsilon_rate;
}

fn part_2(arr: &Vec<Vec<i32>>) -> i32 {
    let mut oxygen_vec = arr.clone();
    for i in 0..oxygen_vec[0].len() {
        // count how often a 1 is in each line
        let oxygen_vec_numbers = oxygen_vec.iter().fold(vec![0; arr[0].len()], |acc, line| {
            acc.iter()
                .zip(line.iter())
                .map(|(a, b)| a + b)
                .collect_vec()
        });
        // filter for the number with more appearance
        oxygen_vec = oxygen_vec
            .iter()
            .filter(|line| {
                if oxygen_vec_numbers[i] >= ((oxygen_vec.len() as i32) - oxygen_vec_numbers[i]) {
                    line[i] == 1
                } else {
                    line[i] == 0
                }
            })
            .cloned()
            .collect::<Vec<Vec<i32>>>();
        if oxygen_vec.len() == 1 {
            break;
        }
    }

    let mut co2_vec = arr.clone();
    for i in 0..co2_vec[0].len() {
        let co2_vec_numbers = co2_vec.iter().fold(vec![0; arr[0].len()], |acc, line| {
            acc.iter()
                .zip(line.iter())
                .map(|(a, b)| a + b)
                .collect_vec()
        });
        co2_vec = co2_vec
            .iter()
            .filter(|line| {
                if co2_vec_numbers[i] < ((co2_vec.len() as i32) - co2_vec_numbers[i]) {
                    return line[i] == 1;
                } else {
                    return line[i] == 0;
                }
            })
            .cloned()
            .collect::<Vec<Vec<i32>>>();

        if co2_vec.len() == 1 {
            break;
        }
    }

    let oxygen_rate =
        isize::from_str_radix(&oxygen_vec[0].iter().join("").to_owned(), 2).unwrap() as i32;
    let co2_rate = isize::from_str_radix(&co2_vec[0].iter().join("").to_owned(), 2).unwrap() as i32;
    return oxygen_rate * co2_rate;
}

fn transform(line: &str) -> Vec<i32> {
    return line
        .chars()
        .map(|it| it as i32 - 0x30)
        .collect::<Vec<i32>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_one() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(part_1(&arr), 198);
    }

    #[test]
    fn test_example_input_part_two() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(part_2(&arr), 230);
    }

    #[test]
    fn test_input_part_one() {
        let arr = parse_file("./input.txt");
        assert_eq!(part_1(&arr), 3882564);
    }

    #[test]
    fn test_input_part_two() {
        let arr = parse_file("./input.txt");
        assert_eq!(part_2(&arr), 3385170);
    }
}
