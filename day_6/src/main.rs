use std::fs;

fn main() {
    let arr = parse_file("./input.txt");
    let fishes = calc_days(&arr, 256);
    println!("Result for 256 days: {}", fishes);
}

fn parse_file(filename: &str) -> Vec<i128> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split(",")
        .map(|it| it.parse::<i128>().unwrap())
        .collect::<Vec<i128>>()
}

fn init_fishes_arr(fishes: &Vec<i128>) -> Vec<i128> {
    let mut new_fishes = vec![0; 9];
    for fish in fishes {
        new_fishes[fish.to_owned() as usize] = new_fishes[fish.to_owned() as usize] + 1;
    }
    new_fishes
}

fn calculate_next_day(fishes: &Vec<i128>) -> Vec<i128> {
    let mut new_fishes = fishes.to_vec();
    let new_fishes_to_add = fishes[0];
    for i in 1..9 {
        new_fishes[i - 1] = new_fishes[i]
    }
    new_fishes[6] = new_fishes[6] + new_fishes_to_add;
    new_fishes[8] = new_fishes_to_add;
    new_fishes
}

fn calc_days(fishes: &Vec<i128>, days: i128) -> i128 {
    let mut new_fishes = init_fishes_arr(fishes);
    for _ in 0..days {
        new_fishes = calculate_next_day(&new_fishes);
    }
    return new_fishes
        .iter()
        .fold(0, |acc, num| acc + num);
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
    fn test_init_fishes() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = init_fishes_arr(&arr);
        assert_eq!(fishes, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn calculate_first_iteration() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = init_fishes_arr(&arr);
        let fist_it = calculate_next_day(&fishes);
        assert_eq!(fist_it, vec![1, 1, 2, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn calculate_second_iteration() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = init_fishes_arr(&arr);
        let fist_it = calculate_next_day(&fishes);
        let fist_it_it = calculate_next_day(&fist_it);
        assert_eq!(fist_it_it, vec![1, 2, 1, 0, 0, 0, 1, 0, 1]);
    }

    #[test]
    fn calculate_day_4() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = calc_days(&arr, 4);
        assert_eq!(fishes, 9);
    }

    #[test]
    fn calculate_day_11() {
        let arr = parse_file("./exampleInput.txt");
        let fishes = calc_days(&arr, 11);
        assert_eq!(fishes, 15);
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
        assert_eq!(fishes, 1754000560399);
    }
}
