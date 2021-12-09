use itertools::*;
use std::{collections::HashMap, fs};

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

fn get_local_minimas(map: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut minimas = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() - 1 {
            if i < map.len() - 1 {
                if i > 0 && j > 0 {
                    if map[i][j] < map[i - 1][j]
                        && map[i][j] < map[i][j - 1]
                        && map[i][j] < map[i + 1][j]
                        && map[i][j] < map[i][j + 1]
                    {
                        minimas.push(map[i][j]);
                    }
                }
                if i == 0 && j > 0 {
                    if map[i][j] < map[i][j - 1]
                        && map[i][j] < map[i + 1][j]
                        && map[i][j] < map[i][j + 1]
                    {
                        minimas.push(map[i][j]);
                    }
                }
                if i > 0 && j == 0 {
                    if map[i][j] < map[i - 1][j]
                        && map[i][j] < map[i + 1][j]
                        && map[i][j] < map[i][j + 1]
                    {
                        minimas.push(map[i][j]);
                    }
                }
                if i == 0 && j == 0 {
                    if map[i][j] < map[i + 1][j] && map[i][j] < map[i][j + 1] {
                        minimas.push(map[i][j]);
                    }
                }
            } else {
                if j > 0 {
                    if map[i][j] < map[i - 1][j]
                        && map[i][j] < map[i][j - 1]
                        && map[i][j] < map[i][j + 1]
                    {
                        minimas.push(map[i][j]);
                    }
                }
                if j == 0 {
                    if map[i][j] < map[i][j + 1] && map[i][j] < map[i-1][j] {
                        minimas.push(map[i][j]);
                    }
                }
            }
        }
        let j = map[i].len() - 1;
        if i < map.len() - 1 {
            if i > 0 {
                if map[i][j] < map[i - 1][j]
                    && map[i][j] < map[i][j - 1]
                    && map[i][j] < map[i + 1][j]
                {
                    minimas.push(map[i][j]);
                }
            }
            if i == 0 {
                if map[i][j] < map[i][j - 1] && map[i][j] < map[i + 1][j] {
                    minimas.push(map[i][j]);
                }
            }
        } else {
            if map[i][j] < map[i - 1][j] && map[i][j] < map[i][j - 1] {
                minimas.push(map[i][j]);
            }
        }
    }

    return minimas;
}

fn get_minimas_part_one(minimas: &Vec<i32>) -> i32 {
    minimas.iter().map(|it| it+1).fold(0, |acc, num| acc+num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_is_parsed_correctly() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_get_local_minimas_example() {
        let arr = parse_file("./exampleInput.txt");
        let minimas = get_local_minimas(&arr);
        assert_eq!(minimas, [1, 0, 5, 5]);
    }

    #[test]
    fn test_get_minimas_part_one_example() {
        let arr = parse_file("./exampleInput.txt");
        let minimas = get_local_minimas(&arr);
        let sum = get_minimas_part_one(&minimas);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_get_minimas_part_one() {
        let arr = parse_file("./input.txt");
        let minimas = get_local_minimas(&arr);
        let sum = get_minimas_part_one(&minimas);
        assert_eq!(sum, 15);
    }
}
