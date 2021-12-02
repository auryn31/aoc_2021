use std::fs;


fn main() {
    let filename = "./input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let arr: Vec<(&str, i32)> = contents
        .split("\n")
        .map(|it|  transform(it))
        .collect();
    let position = part_1(&arr);
    println!("we are at position {}", position);
    let position_2 = part_2(&arr);
    println!("we are at position {}", position_2);
}

fn part_1(arr: &Vec<(&str, i32)>) -> i32 {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    for command in arr {
        match command.0 {
            "up" => vertical_position -= command.1,
            "down" => vertical_position += command.1,
            "forward" => horizontal_position += command.1,
            _ => println!("shouldn't be here")
        }
    }
    return horizontal_position * vertical_position;
}

fn part_2(arr: &Vec<(&str, i32)>) -> i32 {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;
    for command in arr {
        match command.0 {
            "up" => aim -= command.1,
            "down" => aim += command.1,
            "forward" => {
                horizontal_position += command.1;
                vertical_position += aim * command.1;
            },
            _ => println!("shouldn't be here")
        }
    }
    return horizontal_position * vertical_position;
}

fn transform(line: &str) -> (&str, i32) {
    let line_parts = line
        .split(" ")
        .collect::<Vec<&str>>();
    return (
        line_parts[0],
        line_parts[1].parse::<i32>().unwrap()
    );
}