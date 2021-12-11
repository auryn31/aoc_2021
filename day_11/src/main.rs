use std::fs;

fn main() {
    let arr = parse_file("./input.txt");
    // let test_result = calculate_part_one(&arr);
    // println!("Result from part 1: {}", test_result);

    // let test_result_2 = calculate_part_two(&arr);
    // println!("Result from part 2: {}", test_result_2);
}

fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|it| {
            it.split("")
                .filter(|it| it.len() == 1)
                .map(|it| it.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn increase_number(number: i32) -> i32 {
    if number > 0 {
        number + 1
    } else {
        0
    }
}

fn flash(matrix: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let mut matrix_copy = matrix.to_vec();
    let mut flashes = 0;
    let mut i = 0;
    let mut j = 0;
    'outer: loop {
        'inner: loop {
            // println!("i: {}, j: {}", i, j);
            if matrix_copy[i][j] > 9 {
                flashes += 1;
                matrix_copy[i][j] = 0;
                if i > 0 && j > 0 {
                    matrix_copy[i - 1][j - 1] = increase_number(matrix_copy[i - 1][j - 1]);
                }
                if i > 0 {
                    matrix_copy[i - 1][j] = increase_number(matrix_copy[i - 1][j]);
                }
                if i > 0 && j < 9 {
                    matrix_copy[i - 1][j + 1] = increase_number(matrix_copy[i - 1][j + 1]);
                }
                if i < 9 && j > 0 {
                    matrix_copy[i + 1][j - 1] = increase_number(matrix_copy[i + 1][j - 1]);
                }
                if i < 9 {
                    matrix_copy[i + 1][j] = increase_number(matrix_copy[i + 1][j]);
                }
                if i < 9 && j < 9 {
                    matrix_copy[i + 1][j + 1] = increase_number(matrix_copy[i + 1][j + 1]);
                }
                if j > 0 {
                    matrix_copy[i][j - 1] = increase_number(matrix_copy[i][j - 1]);
                }
                if j < 9 {
                    matrix_copy[i][j + 1] = increase_number(matrix_copy[i][j + 1]);
                }
                i = 0;
                j = 0;
            } else {
                j += 1;
                if j > 9 {
                    j = 0;
                    break 'inner;
                }
            }
        }
        i += 1;
        if i > 9 {
            break 'outer;
        }
    }
    return (matrix_copy, flashes);
}

fn step(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matrix_copy = matrix.to_vec();
    for i in 0..10 {
        for j in 0..10 {
            matrix_copy[i][j] += 1;
        }
    }
    return matrix_copy;
}

fn calculate_flashes(matrix: &Vec<Vec<i32>>, itterations: i32) -> i32 {
    let mut flashes = 0;
    let mut current_matrix = matrix.to_vec();
    for _ in 0..itterations {
        current_matrix = step(&current_matrix);
        loop {
            let (new_matrix, flashes_to_add) = flash(&current_matrix);
            current_matrix = new_matrix;
            flashes += flashes_to_add;
            if flashes_to_add == 0 {
                break;
            }
        }
    }
    return flashes;
}

fn find_simultanius_flashes(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut i = 0;
    let mut current_matrix = matrix.to_vec();
    loop {
        current_matrix = step(&current_matrix);
        let mut current_flash_sum = 0;
        i+=1;
        loop {
            let (new_matrix, flashes_to_add) = flash(&current_matrix);
            current_matrix = new_matrix;
            current_flash_sum += flashes_to_add;
            if flashes_to_add == 0 {
                break;
            }
        }
        if current_flash_sum == 100 {
            break;
        }
    }
    return i;
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
    fn test_flashe_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let flashes = flash(&arr);
        assert_eq!(flashes.1, 0);
    }

    #[test]
    fn test_calculate_flashes_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let flashes = calculate_flashes(&arr, 100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_calculate_itterations_test_file() {
        let arr = parse_file("./exampleInput.txt");
        let itterations = find_simultanius_flashes(&arr);
        assert_eq!(itterations, 195);
    }

    #[test]
    fn test_calculate_flashes_file() {
        let arr = parse_file("./input.txt");
        let flashes = calculate_flashes(&arr, 100);
        assert_eq!(flashes, 1739);
    }

    #[test]
    fn test_calculate_itterations_file() {
        let arr = parse_file("./input.txt");
        let itterations = find_simultanius_flashes(&arr);
        assert_eq!(itterations, 324);
    }
}
