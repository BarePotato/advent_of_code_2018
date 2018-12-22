use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let input = input.trim();

    println!("{}", part_one(input));

    println!("{}", part_two(input));
}

fn part_two(polymer: &str) -> usize {
    let mut shortest = 0;

    for chr in b'a'..=b'z' {
        let chr = chr as char;

        let improved = polymer
            .replace(chr, "")
            .replace(chr.to_ascii_uppercase(), "");

        let poly_len = part_one(&improved);

        if poly_len < shortest || shortest == 0 {
            shortest = poly_len;
        }
    }

    shortest
}

fn part_one(polymer: &str) -> usize {
    let mut remains: Vec<char> = Vec::new();

    for chr in polymer.chars() {
        let last = remains.last().unwrap_or(&'0');

        if last.eq_ignore_ascii_case(&chr) && last != &chr {
            remains.pop();
        } else {
            remains.push(chr);
        }
    }

    remains.len()
}
