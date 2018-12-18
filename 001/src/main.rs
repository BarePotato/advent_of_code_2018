use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut freq = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(freq);

    'outer: loop {
        for line in input.lines() {
            freq = freq + line.parse::<i32>().unwrap();

            if seen.contains(&freq) {
                break 'outer;
            } else {
                seen.insert(freq);
            }
        }
    }

    println!("Freq: {}", freq);
}
