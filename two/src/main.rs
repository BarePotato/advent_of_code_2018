use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut two = 0;
    let mut three = 0;
    for line in input.lines() {
        let mut has_two = false;
        let mut has_three = false;

        for chr in line.chars() {
            match line.matches(chr).collect::<Vec<_>>().len() {
                3 => has_three = true,
                2 => has_two = true,
                _ => {}
            }
        }

        if has_three {
            three += 1;
        }
        if has_two {
            two += 1;
        }
    }

    let checksum: usize = two * three;

    println!("Checksum: {}", checksum);
}
