// For instance, r and R are units with the same type but opposite polarity,
// whereas r and s are entirely different types and do not react.
// For example:

// In aA, a and A react, leaving nothing behind.
// In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
// In abAB, no two adjacent units are of the same type, and so nothing happens.
// In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
// Now, consider a larger example, dabAcCaCBAcCcaDA:

// dabAcCaCBAcCcaDA  The first 'cC' is removed.
// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
// dabCBAcaDA        No further actions can be taken.
// After all possible reactions, the resulting polymer contains 10 units.

// How many units remain after fully reacting the polymer you scanned?

use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    println!("{}", part_one(input.trim()));
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
