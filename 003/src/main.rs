use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("{}", part_one(&mut input));
}

fn part_one(input: &mut String) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];
    let mut count = 0;

    for line in input.lines() {
        let v = line.split_whitespace().collect::<Vec<_>>();

        let split = v[2].find(",").unwrap();
        let (x, y) = (
            v[2].get(..split).unwrap().parse::<usize>().unwrap(),
            v[2].get(split + 1..v[2].len() - 1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );

        let split = v[3].find("x").unwrap();
        let (w, h) = (
            v[3].get(..split).unwrap().parse::<usize>().unwrap(),
            v[3].get(split + 1..).unwrap().parse::<usize>().unwrap(),
        );

        for row in x..(x + w) {
            for col in y..(y + h) {
                grid[row][col] += 1;
            }
        }
    }

    grid.iter().flatten().filter(|&&i| i > 1).count()
}
