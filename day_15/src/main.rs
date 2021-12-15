use fast_paths::InputGraph;
use itertools::*;
use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();
    let arr = parse_file("./input.txt");
    let weight_1 = find_path(&arr);
    println!("Result from part 1: {}", weight_1);

    let matrix = enlarge(&arr);
    let weight_2 = find_path(&matrix);
    println!("Result from part 2: {}", weight_2);

    println!("Time: {}ms", now.elapsed().as_millis());
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!(
        "Matrix:\n\n{}\n",
        matrix.iter().map(|it| it.iter().join(" ")).join("\n")
    );
}

fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let matrix: Vec<Vec<i32>> = contents
        .split("\n")
        .map(|it| {
            it.split("")
                .filter(|it| it.len() == 1)
                .map(|it| it.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    return matrix;
}

fn enlarge(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_matrix = vec![vec![0; matrix[0].len() * 5]; matrix.len() * 5];
    let width = matrix[0].len();
    let height = matrix.len();
    for i in 0..new_matrix.len() {
        for j in 0..new_matrix[i].len() {
            new_matrix[i][j] =
                matrix[i % height][j % width] + (j / width) as i32 + (i / height) as i32;

            if new_matrix[i][j] >= 10 {
                new_matrix[i][j] = new_matrix[i][j] % 10 + 1;
            }
        }
    }
    return new_matrix;
}

fn find_path(matrix: &Vec<Vec<i32>>) -> i32 {
    let result;

    let multiplier = matrix.len();

    let mut input_graph = InputGraph::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == matrix.len() - 1 && j < matrix[i].len() - 1 {
                input_graph.add_edge(
                    i * multiplier + j,
                    i * multiplier + j + 1,
                    matrix[i][j + 1] as usize,
                );
            } else if j == matrix[i].len() - 1 && i < matrix.len() - 1 {
                input_graph.add_edge(
                    i * multiplier + j,
                    (i + 1) * multiplier + j,
                    matrix[i + 1][j] as usize,
                );
            } else if j < matrix[i].len() - 1 && i < matrix.len() - 1 {
                input_graph.add_edge(
                    i * multiplier + j,
                    i * multiplier + j + 1,
                    matrix[i][j + 1] as usize,
                );
                input_graph.add_edge(
                    i * multiplier + j,
                    (i + 1) * multiplier + j,
                    matrix[i + 1][j] as usize,
                );
            }
        }
    }
    input_graph.freeze();
    let fast_graph = fast_paths::prepare(&input_graph);
    let shortest_path = fast_paths::calc_path(
        &fast_graph,
        0,
        (matrix.len() - 1) * multiplier + matrix[0].len() - 1,
    );
    match shortest_path {
        Some(p) => {
            // the weight of the shortest path
            result = p.get_weight() as i32;
            // let nodes = p.get_nodes();
        }
        None => {
            panic!("No path found")
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr.len(), 10);
        assert_eq!(arr[0].len(), 10);
    }

    #[test]
    fn test_file_find_path() {
        let arr = parse_file("./exampleInput.txt");
        let weight = find_path(&arr);
        assert_eq!(weight, 40);
    }

    #[test]
    fn test_file_find_path_enlarge() {
        let arr = parse_file("./exampleInput.txt");
        print_matrix(&arr);
        let matrix = enlarge(&arr);
        // print_matrix(&matrix);
        assert_eq!(
            matrix[0],
            vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9,
                6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6
            ]
        );
        assert_eq!(
            matrix,
            parse_file("./exampleInputEnlarged.txt")
        );
        let weight = find_path(&matrix);
        assert_eq!(weight, 315);
    }

    #[test]
    fn test_file_find_path_input() {
        let arr = parse_file("./input.txt");
        let weight = find_path(&arr);
        assert_eq!(weight, 811);
    }

    #[test]
    fn test_file_find_path_input_part_2() {
        let arr = parse_file("./input.txt");
        let matrix = enlarge(&arr);
        let weight = find_path(&matrix);
        assert_eq!(weight, 3012);
    }
}
