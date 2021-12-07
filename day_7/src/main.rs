use std::fs;

fn main() {
    let arr = parse_file("./input.txt");
}

fn parse_file(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split(",")
        .map(|it| it.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn calculate_position(crabs: &Vec<i32>) -> i32 {
    let mut crabs_to_sort = crabs.to_vec();
    crabs_to_sort.sort();
    let mid = crabs_to_sort.len() / 2;
    crabs_to_sort[mid]
}

fn calculate_position_part_two(crabs: &Vec<i32>) -> i32 {
    ((crabs.iter().fold(0, |acc, num| acc + num) as f64) / crabs.len() as f64).round() as i32
}

fn calculate_costs(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs
        .iter()
        .map(|it| if it > &pos { it - pos } else { pos - it })
        .fold(0, |acc, num| acc + num)
}

fn calculate_costs_part_two(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs
        .iter()
        .map(|it| if it > &pos { it - pos } else { pos - it })
        .map(|it| (it * (it + 1)) / 2)
        .fold(0, |acc, num| acc + num)
}

fn brute_force_part_two(crabs: &Vec<i32>, start_position: i32) -> i32 {
    let mut costs = calculate_costs_part_two(crabs, start_position);
    let mut pos = start_position;
    loop {
        pos += 1;
        let new_costs = calculate_costs_part_two(crabs, pos);
        if new_costs < costs {
            costs = new_costs;
        } else {
            break;
        }
    }

    pos = start_position;
    loop {
        pos -= 1;
        let new_costs = calculate_costs_part_two(crabs, pos);
        if new_costs < costs {
            costs = new_costs;
        } else {
            break;
        }
    }
    return costs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testfile_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_position_calculation_for_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let pos = calculate_position(&arr);
        assert_eq!(pos, 2);
    }

    #[test]
    fn test_calculate_costs_for_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let pos = calculate_position(&arr);
        let costs = calculate_costs(&arr, pos);
        assert_eq!(costs, 37);
    }

    #[test]
    fn test_calculate_costs_for_test_file_for_part_two() {
        let arr = parse_file("./exampleInput.txt");
        let pos = calculate_position_part_two(&arr);
        let costs = brute_force_part_two(&arr, pos);
        assert_eq!(costs, 168);
    }

    #[test]
    fn test_calculate_costs_for_file() {
        let arr = parse_file("./input.txt");
        let pos = calculate_position(&arr);
        let costs = calculate_costs(&arr, pos);
        assert_eq!(costs, 331067);
    }

    #[test]
    fn test_calculate_costs_for_file_for_part_two() {
        let arr = parse_file("./input.txt");
        let pos = calculate_position_part_two(&arr);
        let costs = brute_force_part_two(&arr, pos);
        assert_eq!(costs, 92881128);
    }
}
