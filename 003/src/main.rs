use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let rects = parse_to_rects(&mut input);

    println!("{}", part_one(&rects));

    println!("{}", part_two(rects));
}

fn part_two(rects: Vec<Rect>) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];
    let all = rects.iter().map(|rect| rect.id).collect::<HashSet<_>>();
    let mut overlaps = HashSet::new();

    for rect in rects {
        for row in rect.x..(rect.x + rect.w) {
            for col in rect.y..(rect.y + rect.h) {
                let claim = grid[row][col];

                if claim > 0 {
                    overlaps.insert(claim);
                    overlaps.insert(rect.id);
                }

                grid[row][col] = rect.id;
            }
        }
    }

    *all.difference(&overlaps).collect::<Vec<_>>()[0]
}

fn part_one(rects: &Vec<Rect>) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];

    for rect in rects {
        for row in rect.x..(rect.x + rect.w) {
            for col in rect.y..(rect.y + rect.h) {
                grid[row][col] += 1;
            }
        }
    }

    grid.iter().flatten().filter(|&&i| i > 1).count()
}

struct Rect {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

// 0 1    2       3
//#1 @ 935,649: 22x22
fn parse_to_rects(input: &mut String) -> Vec<Rect> {
    let mut rects = Vec::new();

    for line in input.lines() {
        let v = line.split_whitespace().collect::<Vec<_>>();

        let id = v[0].get(1..).unwrap().parse().unwrap();

        let split = v[2].find(",").unwrap();
        let (x, y) = (
            v[2].get(..split).unwrap().parse().unwrap(),
            v[2].get(split + 1..v[2].len() - 1)
                .unwrap()
                .parse()
                .unwrap(),
        );

        let split = v[3].find("x").unwrap();
        let (w, h) = (
            v[3].get(..split).unwrap().parse().unwrap(),
            v[3].get(split + 1..).unwrap().parse().unwrap(),
        );

        rects.push(Rect { id, x, y, w, h });
    }

    rects
}
