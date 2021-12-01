use std::fs;

fn main() {
    let contents = fs::read_to_string("input");
    let mut increases = 0;
    let file_string = contents.unwrap();
    let depths: Vec<&str> = file_string.split('\n').collect();
    let mut back = 0;
    let mut front = 3;
    while front < depths.len() {
        let back_depth = depths[back].parse::<u64>().unwrap();
        let front_depth = depths[front].parse::<u64>().unwrap();
        if back_depth < front_depth {
            increases += 1;
        }
        front += 1;
        back += 1;
    }
    println!("Increases: {}", increases);
}