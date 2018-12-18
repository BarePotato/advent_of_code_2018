use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("{}", part_one(&mut input));

    println!("{}", part_two(&mut input));
}

fn part_two(input: &mut String) -> String {
    let mut lines = input.lines().collect::<Vec<_>>();
    let mut first = "";

    loop {
        if lines.len() > 0 {
            first = lines.remove(0);
        } else {
            panic!("It's all wrong, Pete Tong!!!");
        }

        for line in &lines {
            let mut v = String::new();
            let mut not_matches = 0;

            for (chr_a, chr_b) in first.chars().zip(line.chars()) {
                if chr_a == chr_b {
                    v.push(chr_a);
                } else {
                    not_matches += 1;
                }

                if not_matches > 1 {
                    break;
                }
            }

            if not_matches == 1 {
                return v;
            }
        }
    }
}

fn part_one(input: &mut String) -> usize {
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

    two * three
}
