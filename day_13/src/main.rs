use itertools::*;
use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();
    // let arr = parse_file("./input.txt");
    // let result = search("start".to_owned(), &arr, "".to_string());
    // let test_result = result.len();
    // println!("Result from part 1: {}", test_result);

    // let result_2 = search_twice("start".to_owned(), &arr, "".to_string());
    // let test_result_2 = result_2.len();
    // println!("Result from part 2: {}", test_result_2);
    // println!("Time: {}ms", now.elapsed().as_millis());
}

struct Input {
    matrix: Vec<Vec<i32>>,
    instructions: Vec<(String, i32)>,
}

fn printMatrix(matrix: &Vec<Vec<i32>>) {
    println!(
        "Matrix:\n{}",
        matrix
            .iter()
            .map(|it| it
                .iter()
                .map(|val| if val == &0 { "." } else { "#" })
                .join(""))
            .join("\n")
    );
}

fn parse_file(filename: &str) -> Input {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let parts: Vec<String> = contents.split("\n\n").map(|it| it.to_owned()).collect();
    let mut dots: Vec<(i32, i32)> = parts[0]
        .split("\n")
        .map(|it| {
            it.split(",")
                .map(|element| element.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let folds: Vec<(String, i32)> = parts[1]
        .split("\n")
        .map(|it| {
            let instruction: Vec<String> = it
                .to_string()
                .replace("fold along ", "")
                .split("=")
                .map(|it| it.to_owned())
                .collect();
            return (
                instruction[0].to_string(),
                instruction[1].parse::<i32>().unwrap(),
            );
        })
        .collect();

    let max_x = dots.iter().map(|it| it.0).max().unwrap() as usize;
    let max_y = dots.iter().map(|it| it.1).max().unwrap() as usize;

    let mut matrix = vec![vec![0; max_x + 1]; max_y + 1];
    for dot in dots {
        matrix[dot.1 as usize][dot.0 as usize] = 1;
    }

    printMatrix(&matrix);

       return Input{
           matrix: matrix,
           instructions: folds
       };
}

fn fold(matrix: &Vec<Vec<i32>>, fold: &(String, i32)) -> Vec<Vec<i32>> {
    let mut matrix_copy = if fold.0 == "y" {
        vec![vec![0; matrix[0].len()]; matrix.len()/2]
    } else {
        vec![vec![0; matrix[0].len()/2]; matrix.len()]
    };
    if fold.0 == "y" {
        for x in 0..matrix_copy.len()  {
            for y in 0..matrix_copy[0].len() {
                matrix_copy[x][y] = matrix[x][y] + matrix[matrix.len()-x-1][y];
            }
        }
    } else {
        for x in 0..matrix_copy.len()  {
            for y in 0..matrix_copy[0].len() {
                matrix_copy[x][y] = matrix[x][y] + matrix[x][matrix[y].len()-y-1];
            }
        }
    }
    return matrix_copy;
}

fn sum(matrix: &Vec<Vec<i32>>) -> i32 {
    matrix.iter().flat_map(|it| it).filter(|it| it.to_owned() != &0).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr.matrix.len(), 15);
    }

    #[test]
    fn test_first_fold() {
        let arr = parse_file("./exampleInput.txt");
        let first_fold = fold(&arr.matrix, &arr.instructions[0]);
        let sum = sum(&first_fold);
        assert_eq!(sum, 17);
    }

    #[test]
    fn test_first_fold_input() {
        let arr = parse_file("./input.txt");
        let first_fold = fold(&arr.matrix, &arr.instructions[0]);
        let sum = sum(&first_fold);
        assert_eq!(sum, 765);
    }

    #[test]
    fn test_finish_folding() {
        let arr = parse_file("./input.txt");
        let mut result = arr.matrix.to_vec();
        for instruction in arr.instructions {
            result = fold(&result, &instruction);
        }
        printMatrix(&result);
        let sum = sum(&result);
        assert_eq!(sum, 765);
    }
}
