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
    let mut counter = 0;
    for i in 1..arr.len() {
        if arr[i-1]<arr[i] {
            counter+=1;
        }
    }
    return counter;
}

fn sliding_window(arr: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in 1..arr.len()-2 {
        let window_1 = arr[i-1]+arr[i]+arr[i+1];
        let window_2 = arr[i]+arr[i+1]+arr[i+2];
        if window_1<window_2 {
            counter+=1;
        }
    }
    return counter;
}
