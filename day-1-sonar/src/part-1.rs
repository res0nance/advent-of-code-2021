use std::fs;

fn main() {
    let contents = fs::read_to_string("input");
    let mut curr = u64::MAX;
    let mut increases = 0;
    for depth in contents.unwrap().split('\n') {
        let dep = depth.parse::<u64>().unwrap();
        if dep > curr {
            increases += 1;
        }
        curr = dep;
    }
    println!("Increases: {}", increases);
}