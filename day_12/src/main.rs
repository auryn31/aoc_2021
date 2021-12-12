use itertools::*;
use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();
    let arr = parse_file("./input.txt");
    let result = search("start".to_owned(), &arr, "".to_string());
    let test_result = result.len();
    println!("Result from part 1: {}", test_result);

    let result_2 = search_twice("start".to_owned(), &arr, "".to_string());
    let test_result_2 = result_2.len();
    println!("Result from part 2: {}", test_result_2);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn parse_file(filename: &str) -> Vec<(String, String)> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|it| {
            it.split("-")
                .filter(|it| it.len() > 0)
                .map(|it| return it.to_owned())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(String, String)>>()
}

fn search(
    start_node: String,
    connections: &Vec<(String, String)>,
    current_path: String,
) -> Vec<String> {
    if start_node == "end".clone() {
        return vec![format!("{},{}", current_path, start_node)];
    }

    let target_caves = connections
        .iter()
        .filter(|it| it.0.clone() == start_node || it.1.clone() == start_node)
        .map(|it| {
            if it.0.clone() == start_node {
                it.1.clone()
            } else {
                it.0.clone()
            }
        })
        .filter(|it| it.clone() != "start")
        .collect::<Vec<String>>();
    let mut paths = Vec::new();
    let visited: Vec<String> = current_path
        .split(",")
        .filter(|it| it.clone() != "start" && it.clone().to_uppercase() != it.clone())
        .map(|it| it.to_string())
        .collect::<Vec<String>>();
    for cave in target_caves {
        if cave.to_uppercase() == cave.clone() || !visited.contains(&cave) {
            let next_current_path = if current_path.len() > 0 {
                format!("{},{}", current_path, start_node.clone())
            } else {
                start_node.clone()
            };
            let mut path = search(cave, connections, next_current_path);
            paths.append(&mut path);
        }
    }
    return paths;
}

fn search_twice(
    start_node: String,
    connections: &Vec<(String, String)>,
    current_path: String,
) -> Vec<String> {
    if start_node == "end".clone() {
        return vec![format!("{},{}", current_path, start_node)];
    }

    let target_caves = connections
        .iter()
        .filter(|it| it.0.clone() == start_node || it.1.clone() == start_node)
        .map(|it| {
            if it.0.clone() == start_node {
                it.1.clone()
            } else {
                it.0.clone()
            }
        })
        .filter(|it| it.clone() != "start")
        .collect::<Vec<String>>();
    let mut paths = Vec::new();
    let visited: Vec<String> = current_path
        .split(",")
        .filter(|it| it.clone() != "start" && it.clone().to_uppercase() != it.clone())
        .map(|it| it.to_string())
        .collect::<Vec<String>>();
    let mut contains_twice = false;
    for visit in &visited {
        if visited.iter().filter(|&n| n == visit).count() > 1 {
            contains_twice = true;
        }
    }

    for cave in target_caves {
        if cave.to_uppercase() == cave.clone() || !contains_twice || !visited.contains(&cave) {
            let next_current_path = format!("{},{}", current_path, cave);
            let mut path = search_twice(cave, connections, next_current_path);
            paths.append(&mut path);
        }
    }
    return paths;
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
    fn test_file_test_path() {
        let arr = parse_file("./exampleInput.txt");
        let results = search("start".to_owned(), &arr, "".to_string());
        assert_eq!(results.len(), 19);
    }

    #[test]
    fn test_file_test_path_large() {
        let arr = parse_file("./exampleInputLarge.txt");
        let results = search("start".to_owned(), &arr, "".to_string());
        assert_eq!(results.len(), 226);
    }

    #[test]
    fn test_file_part_one() {
        let arr = parse_file("./input.txt");
        let results = search("start".to_owned(), &arr, "".to_string());
        assert_eq!(results.len(), 3679);
    }

    #[test]
    fn test_file_test_path_small_part_two() {
        let arr = parse_file("./exampleInputSmall.txt");
        let results = search_twice("start".to_owned(), &arr, "".to_string());
        println!("results: {}", results.iter().join("\n"));
        assert_eq!(results.len(), 36);
    }

    #[test]
    fn test_file_test_path_mid_part_two() {
        let arr = parse_file("./exampleInput.txt");
        let results = search_twice("start".to_owned(), &arr, "".to_string());
        assert_eq!(results.len(), 103);
    }

    #[test]
    fn test_file_test_path_large_part_two() {
        let arr = parse_file("./exampleInputLarge.txt");
        let results = search_twice("start".to_owned(), &arr, "".to_string());
        assert_eq!(results.len(), 3509);
    }

    #[test]
    fn test_file_part_two() {
        let arr = parse_file("./input.txt");
        let results = search_twice("start".to_owned(), &arr, "".to_string());
        assert_eq!(results.len(), 107395);
    }
}
