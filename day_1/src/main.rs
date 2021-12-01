use std::fs;

fn main() {
    let filename = "./input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let arr: Vec<i32> = contents
        .split("\n")
        .map(|it| it.parse::<i32>().unwrap())
        .collect();
    let deeper_water = count_deeper_water(&arr);
    println!("There are {} deeper counts", deeper_water);
    let sliding_windwow_result = sliding_window(&arr);
    println!("There are {} deeper counts", sliding_windwow_result);
}

fn count_deeper_water(arr: &Vec<i32>) -> i32 {
    return arr
        .windows(2)
        .map(|it| if it[0] < it[1] { 1 } else { 0 })
        .sum();
}

fn sliding_window(arr: &Vec<i32>) -> i32 {
    let window_sum = arr
        .windows(3)
        .map(|it| it.iter().sum())
        .collect::<Vec<i32>>();
    return count_deeper_water(&window_sum);
}
