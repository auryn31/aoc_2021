use std::fs;

use itertools::*;
fn main() {
    println!("Run cargo test for the results");
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
    let x = input.iter().flat_map(|it| vec![it.0.0, it.1.0]).max().unwrap() as usize;
    let y = input.iter().flat_map(|it| vec![it.0.1, it.1.1]).max().unwrap() as usize;
    return vec![vec![0; x+1]; y+1];
}

fn create_range(from: i32, to: i32) -> Vec<i32> {
    if from > to {
        (to..from+1).rev().collect::<Vec<i32>>()
    } else if from < to {
        (from..to+1).collect::<Vec<i32>>()
    } else {
        vec![]
    }
}

fn create_points_from_line(line:((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let mut horizontal = create_range(line.0.0, line.1.0);
    let mut vertical: Vec<i32> = create_range(line.0.1, line.1.1);
    if vertical.len() == 0 {
        vertical = vec![line.0.1; horizontal.len()]
    }
    if horizontal.len() == 0 {
        horizontal = vec![line.0.0; vertical.len()]
    }
    let numbers = horizontal.iter()
    .zip(vertical.iter())
    .inspect(|f| println!("{}, {}", f.0, f.1))
    .map(|it| (it.0.to_owned(), it.1.to_owned()))
    .collect::<Vec<(i32, i32)>>();
    return numbers;
}

fn fill_matrix(input: ((i32, i32), (i32, i32)), matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_matrix = matrix.to_vec();
    let points = create_points_from_line(input);
    for point in points {
        new_matrix[point.1 as usize][point.0 as usize] = new_matrix[point.1 as usize][point.0 as usize] + 1
    }
    return new_matrix;
}

fn fill_complete_matrix_part_two(input: &Vec<((i32, i32), (i32, i32))>, matrix: Vec<Vec<i32>>)-> Vec<Vec<i32>> {
    let mut new_matrix = matrix.to_vec();
    for line in input {
        new_matrix = fill_matrix(*line, new_matrix);
    }
    return new_matrix;
}

fn fill_complete_matrix(input: &Vec<((i32, i32), (i32, i32))>, matrix: Vec<Vec<i32>>)-> Vec<Vec<i32>> {
    let mut new_matrix = matrix.to_vec();
    for line in input {
        if line.0.0 == line.1.0 || line.0.1 == line.1.1 {
            new_matrix = fill_matrix(*line, new_matrix);
        }
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
        println!("{}", matrix_new.iter().map(|it| it.iter().map(|it| it.to_string()).map(|it| if it.eq("0") {".".to_string()} else {it}).join(",")).join("\n"));
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

    #[test]
    fn calculate_test_file_part_two() {
        let arr = parse_file("./exampleInput.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_complete_matrix_part_two(&arr, matrix);
        println!("{}", matrix_new.iter().map(|it| it.iter().map(|it| it.to_string()).map(|it| if it.eq("0") {".".to_string()} else {it}).join("")).join("\n"));
        let result = calculate_part_one(matrix_new);
        assert_eq!(result, 12);
    }

    #[test]
    fn calculate_input_file_part_two() {
        let arr = parse_file("./input.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_complete_matrix_part_two(&arr, matrix);
        let result = calculate_part_one(matrix_new);
        assert_eq!(result, 18423);
    }

    #[test]
    fn calculate_input_file_part_two_for_kay() {
        let arr = parse_file("./input_kay.txt");
        let matrix = create_matrix(&arr);
        let matrix_new = fill_complete_matrix_part_two(&arr, matrix);
        let result = calculate_part_one(matrix_new);
        assert_eq!(result, 18864);
    }

    #[test]
    fn test_create_points_from_line() {
        let points = create_points_from_line(((8,0), (0,8)));
        assert_eq!(points.len(), 9);
        assert_eq!(points[0], (8,0));
        assert_eq!(points[1], (7,1));
        assert_eq!(points[2], (6,2));
        assert_eq!(points[3], (5,3));
    }

    #[test]
    fn test_create_points_from_line_horizontal() {
        let points = create_points_from_line(((0,9), (5,9)));
        assert_eq!(points.len(), 6);
        assert_eq!(points[0], (0,9));
        assert_eq!(points[1], (1,9));
        assert_eq!(points[2], (2,9));
        assert_eq!(points[3], (3,9));
    }

    #[test]
    fn create_range_test() {
        let range = create_range(3, 5);
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], 3);
        assert_eq!(range[1], 4);
        assert_eq!(range[2], 5);
    }

    #[test]
    fn create_range_test_reverse() {
        let range = create_range(5, 3);
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], 5);
        assert_eq!(range[1], 4);
        assert_eq!(range[2], 3);
    }
}
