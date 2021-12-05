use std::fmt;
use std::fs;

use itertools::*;
fn main() {
    let filename = "./input.txt";
    run_for_file(filename);
}

fn run_for_file(filename: &str) {
    println!("In file {}", filename);
    let arr = parse_file(filename);
    // let position = part_1(&arr.0, &arr.1);
    // println!("part 1 {}", position);
    // let position_2 = part_2(&arr.0, &arr.1);
    // println!("part 2 {}", position_2);
}



fn parse_file(filename: &str) -> Vec<((i32, i32), (i32, i32))> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.split("\n")
    .map(|it| {
        it.split(" -> ").map(|it| {
            it.split(",")
            .map(|it| it.parse::<i32>().unwrap())
            .collect_tuple().unwrap()
        }).collect_tuple().unwrap()
    }).collect::<Vec<((i32, i32), (i32, i32))>>()
}

fn create_matrix(input: &Vec<((i32, i32), (i32, i32))>) -> Vec<Vec<i32>> {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for line in input {
        if line.0.0 > x as i32 {
            x = line.0.0 as usize
        }
        if line.1.0 > x as i32 {
            x = line.1.0 as usize
        }

        if line.0.1 > y as i32 {
            y = line.0.1 as usize
        }
        if line.1.1 > y as i32 {
            y = line.1.1 as usize
        }
    }
    return vec![vec![0; x+1]; y+1];
}

fn fill_matrix(input: ((i32, i32), (i32, i32)), matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_matrix = matrix.to_vec();
    if input.0.1 == input.1.1 {
        if input.0.0 < input.1.0 {
            for i in (input.0.0 as usize)..((input.1.0 + 1) as usize) {
                // println!("update position ({}, {}) to number {}", i, input.0.1, new_matrix[i][input.0.1 as usize] + 1);
                new_matrix[input.0.1 as usize][i] = new_matrix[input.0.1 as usize][i] + 1;
            }
        } else {
            for i in (input.1.0 as usize)..((input.0.0 + 1) as usize) {
                // println!("update position ({}, {}) to number {}", i, input.0.1, new_matrix[i][input.0.1 as usize] + 1);
                new_matrix[input.0.1 as usize][i] = new_matrix[input.0.1 as usize][i] + 1;
            }
        }
    }
    if input.0.0 == input.1.0 {
        if input.0.1 < input.1.1 {
            for i in (input.0.1 as usize)..((input.1.1 + 1) as usize) {
                // println!("update position ({}, {}) to number {}", i, input.0.1, new_matrix[i][input.0.1 as usize] + 1);
                new_matrix[i][input.0.0 as usize] = new_matrix[i][input.0.0 as usize] + 1;
            }
        } else {
            for i in ((input.1.1) as usize)..((input.0.1 + 1) as usize) {
                // println!("update position ({}, {}) to number {}", i, input.0.1, new_matrix[i][input.0.1 as usize] + 1);
                new_matrix[i][input.0.0 as usize] = new_matrix[i][input.0.0 as usize] + 1;
            }
        }
    }
    return new_matrix;
}

fn fill_complete_matrix(input: &Vec<((i32, i32), (i32, i32))>, matrix: Vec<Vec<i32>>)-> Vec<Vec<i32>> {
    let mut new_matrix = matrix.to_vec();
    for line in input {
        // println!("update for line ({},{}) -> ({}, {})", line.0.0, line.0.1, line.1.0, line.1.1);
        new_matrix = fill_matrix(*line, new_matrix);
    }
    return new_matrix;
}

fn calculate_part_one(matrix: Vec<Vec<i32>>) -> i32 {
    matrix
    .iter()
    .flat_map(|it| it.to_owned())
    .filter(|it| it.to_owned() > 1)
    .count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testfile_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr[0].0.0, 0);
        assert_eq!(arr[0].0.1, 9);
        assert_eq!(arr[0].1.0, 5);
        assert_eq!(arr[0].1.1, 9);
    }

    #[test]
    fn matrix_is_created_correctrly() {
        let arr = parse_file("./exampleInput.txt");
        let matrix = create_matrix(&arr);
        assert_eq!(matrix.len(), 10);
        assert_eq!(matrix[0].len(), 10);
    }

    #[test]
    fn matrix_is_filled_correctrly() {
        let arr = parse_file("./exampleInput.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_matrix(arr[0], matrix);
        println!("{}", matrix_new.iter().map(|it| it.iter().join(",")).join("\n"));
        assert_eq!(matrix_new.len(), 10);
        assert_eq!(matrix_new[9][0], 1);
    }

    #[test]
    fn matrix_is_filled_correctrly_completly() {
        let arr = parse_file("./exampleInput.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_complete_matrix(&arr, matrix);
        println!("{}", matrix_new.iter().map(|it| it.iter().join(",")).join("\n"));
        assert_eq!(matrix_new.len(), 10);
        assert_eq!(matrix_new[9][0], 2);
    }

    #[test]
    fn calculate_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_complete_matrix(&arr, matrix);
        println!("{}", matrix_new.iter().map(|it| it.iter().join(",")).join("\n"));
        let result = calculate_part_one(matrix_new);
        assert_eq!(result, 5);
    }

    #[test]
    fn calculate_input_file() {
        let arr = parse_file("./input.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_complete_matrix(&arr, matrix);
        let result = calculate_part_one(matrix_new);
        assert_eq!(result, 5774);
    }

}
