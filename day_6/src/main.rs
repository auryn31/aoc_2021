use std::fs;

use itertools::*;
fn main() {
    let arr = parse_file("./input.txt");
    let fishes = calc_days(&arr, 256);
    println!("Result for 256 days: {}", fishes);
}

fn parse_file(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split(",")
        .map(|it| it.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn calculate_next_day(fishes: &Vec<i32>) -> Vec<i32> {
    let mut new_fishes = fishes.to_vec();
    for i in 0..new_fishes.len() {
        if fishes[i] == 0 {
            new_fishes[i] = 6;
            new_fishes.push(8);
        } else {
            new_fishes[i] = new_fishes[i] - 1;
        }
    }
    new_fishes
}

fn calc_days(fishes: &Vec<i32>, days: i32) -> i64 {
    let mut new_fishes = fishes.to_vec();
    for _ in 0..days {
        new_fishes = calculate_next_day(&new_fishes);
    }
    return new_fishes.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testfile_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn calculate_first_iteration() {
        let arr = parse_file("./exampleInput.txt");
        let fist_it = calculate_next_day(&arr);
        assert_eq!(fist_it, vec![2, 3, 2, 0, 1]);
    }

    #[test]
    fn calculate_second_iteration() {
        let arr = parse_file("./exampleInput.txt");
        let fist_it = calculate_next_day(&arr);
        let second_it = calculate_next_day(&fist_it);
        assert_eq!(second_it, vec![1, 2, 1, 6, 0, 8]);
    }

    #[test]
    fn calucalte_test_size() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = calc_days(&arr, 80);
        assert_eq!(fishes, 5934);
    }

    #[test]
    fn calucalte_test_size_for_256() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = calc_days(&arr, 256);
        assert_eq!(fishes, 26984457539);
    }

    #[test]
    fn calucalte_size() {
        let arr = parse_file("./input.txt");
        let fishes = calc_days(&arr, 256);
        assert_eq!(fishes, 391671);
    }

    #[test]
    fn calucalte_size_day_2() {
        let arr = parse_file("./input.txt");
        let fishes = calc_days(&arr, 256);
        assert_eq!(fishes, 391671);
    }
}
