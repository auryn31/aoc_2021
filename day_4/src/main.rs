use std::fmt;
use std::{fs, io::BufRead};

use itertools::*;
fn main() {
    let filename = "./input.txt";
    run_for_file(filename);
}

fn run_for_file(filename: &str) {
    println!("In file {}", filename);
    let arr = parse_file(filename);
    let position = part_1(arr.0, arr.1);
    println!("part 1 {}", position);
    // let position_2 = part_2(arr.0, arr.1);
    // println!("part 2 {}", position_2);
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<i32>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("");

        for line in &self.board {
            res.push_str(format!("{}\n", line.iter().join(",")).as_str());
        }
        write!(f, "{}", res)
    }
}

fn parse_file(filename: &str) -> (Vec<i32>, Vec<Board>) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let file_input = contents.split("\n\n").collect::<Vec<&str>>();

    let bingo_numbers: Vec<i32> = file_input[0]
        .split(",")
        .map(|it| it.parse::<i32>().unwrap())
        .collect();

    let boards: Vec<Board> = file_input
        .iter()
        .skip(1)
        .map(|it| parse_board(it))
        .collect();
    return (bingo_numbers, boards);
}

fn parse_board(board_raw: &str) -> Board {
    let mut board: Vec<Vec<i32>> = Vec::new();
    for board_line in board_raw.split("\n").collect::<Vec<&str>>() {
        let line: Vec<i32> = board_line
            .split(" ")
            .filter(|it| !it.is_empty())
            .map(|it| it.parse::<i32>().unwrap())
            .collect();
        board.push(line);
    }
    return Board { board: board };
}

fn board_wins(numbers: Vec<i32>, board: &Board) -> bool {
    for row in &board.board {
        if !row
            .iter()
            .filter(|it| numbers.contains(it))
            .collect::<Vec<&i32>>()
            .len()
            == 0
        {
            return true;
        };
    }

    for i in 0..board.board[0].len() {
        let mut is_winning_column = true;
        for j in 0..board.board.len() {
            if !numbers.contains(&board.board[j][i]) {
                is_winning_column = false;
            }
        }
        if is_winning_column {
            return true;
        }
    }
    false
}

fn calculate_part_one(numbers: Vec<i32>, winning_number: i32, board: &Board) -> i32 {
    let sum_unmarked = board
        .board
        .iter()
        .flat_map(|it| it.clone())
        .filter(|it| !numbers.contains(it))
        .fold(0, |acc, num| acc + num);
    println!(
        "Undmarked: {}",
        board
            .board
            .iter()
            .flat_map(|it| it.clone())
            .filter(|it| !numbers.contains(it))
            .join(",")
    );
    return sum_unmarked * winning_number;
}

fn part_1(numbers: Vec<i32>, boards: Vec<Board>) -> i32 {
    for i in 0..numbers.len() {
        for board in &boards {
            if board_wins(numbers[0..i].to_vec(), board) {
                return calculate_part_one(numbers[0..i].to_vec(), numbers[i - 1], board);
            }
        }
    }
    return -1;
}

fn part_2(numbers: Vec<i32>, boards: Vec<Board>) -> i32 {
    let mut boards_copy = boards.to_vec();
    for i in 0..numbers.len() {
        if boards_copy.len() == 1 {
            println!("winning number: {}", numbers[i - 1]);
            println!(
                "Numbers drwan before: {}",
                numbers[0..i].to_vec().iter().join(",")
            );
            println!("winning board: \n{}", boards_copy[0]);
            println!("leftover boards: {}", boards_copy.len());

            return calculate_part_one(
                numbers[0..i].to_vec(),
                numbers[i - 1],
                &boards_copy.last().unwrap(),
            );
        } else {
            boards_copy = boards_copy
                .iter()
                .filter(|it| !board_wins(numbers[0..i].to_vec(), it))
                .map(|it| it.clone())
                .collect::<Vec<Board>>();
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_one() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(part_1(arr.0, arr.1), 4512);
    }

    #[test]
    fn test_if_board_wins_on_row() {
        let wins = board_wins(
            vec![1, 2, 3, 4, 5, 5, 6],
            &Board {
                board: vec![
                    vec![1, 2, 3, 4, 5, 5, 6],
                    vec![1, 2, 3, 4, 5, 5, 6],
                    vec![1, 2, 3, 4, 5, 5, 6],
                    vec![1, 2, 3, 4, 5, 5, 6],
                ],
            },
        );
        assert_eq!(wins, true);
    }

    #[test]
    fn test_if_board_wins_on_column() {
        let wins = board_wins(
            vec![1, 2, 3, 4],
            &Board {
                board: vec![
                    vec![1, 1, 1, 1, 1],
                    vec![2, 2, 2, 2, 2, 2],
                    vec![3, 3, 3, 3, 3],
                    vec![4, 4, 4, 4],
                ],
            },
        );
        assert_eq!(wins, true);
    }

    #[test]
    fn test_if_board_wins_on_column_next() {
        let wins = board_wins(
            vec![
                17, 2, 33, 86, 38, 41, 4, 34, 91, 61, 11, 81, 3, 59, 29, 71, 26, 44, 54, 89, 46, 9,
                85, 62, 23, 76, 45, 24, 78, 14, 58, 48, 57, 40, 21, 49, 7, 99, 8, 56, 50, 19, 53,
                55, 10, 94, 75, 68, 6, 83, 84, 88, 52, 80, 73, 74, 79, 36, 70, 28, 37, 0, 42, 98,
                96, 92, 27, 90, 47, 20, 5, 77, 69, 93, 31, 30, 95, 25, 63, 65, 51, 72, 60, 16, 12,
                64, 18, 13,
            ],
            &Board {
                board: vec![
                    vec![3, 38, 22, 72, 80],
                    vec![56, 48, 1, 50, 60],
                    vec![49, 98, 67, 53, 30],
                    vec![79, 61, 66, 9, 45],
                    vec![96, 24, 23, 43, 78],
                ],
            },
        );
        assert_eq!(wins, true);
    }

    #[test]
    fn test_if_board_wins_on_column_next_2() {
        let wins = board_wins(
            vec![38, 48, 98, 61, 24],
            &Board {
                board: vec![
                    vec![3, 38, 22, 72, 80],
                    vec![56, 48, 1, 50, 60],
                    vec![49, 98, 67, 53, 30],
                    vec![79, 61, 66, 9, 45],
                    vec![96, 24, 23, 43, 78],
                ],
            },
        );
        assert_eq!(wins, true);
    }

    #[test]
    fn test_if_board_not_wins_on_column() {
        let wins = board_wins(
            vec![1, 2, 3, 4],
            &Board {
                board: vec![
                    vec![1, 6, 3, 6, 4],
                    vec![2, 46, 23, 2, 6, 623],
                    vec![5, 6, 23, 6243, 2346],
                    vec![4, 234, 462, 234],
                ],
            },
        );
        assert_eq!(wins, false);
    }

    #[test]
    fn test_example_input_part_two() {
        let arr = parse_file("./exampleInput.txt");
        assert_eq!(part_2(arr.0, arr.1), 1923);
    }

    #[test]
    fn test_input_part_one() {
        let arr = parse_file("./input.txt");
        assert_eq!(part_1(arr.0, arr.1), 38594);
    }

    #[test]
    fn test_input_part_two() {
        let arr = parse_file("./input.txt");
        assert_eq!(part_2(arr.0, arr.1), 1981);
    }
}
