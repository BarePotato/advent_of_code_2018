use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut vec = input.lines().collect::<Vec<_>>();
    vec.sort();

    let guards = parse_to_guards(vec);

    println!("{}", part_one(&guards));
}

fn part_one(guards: &HashMap<usize, Vec<usize>>) -> usize {
    let mut sleeper = 0;
    let mut total = 0;

    for (guard, minutes) in guards {
        let sum = minutes.iter().sum::<usize>();

        if sum > total {
            sleeper = *guard;
            total = sum;
        }
    }

    let (mut idx_long, mut longest) = (0, 0);
    for (i, minute) in guards.get(&sleeper).unwrap().iter().enumerate() {
        if minute > &longest {
            idx_long = i;
            longest = *minute;
        }
    }

    sleeper * idx_long
}

// [1518-02-13 00:00] Guard #2969 begins shift
// [1518-02-13 00:32] falls asleep
// [1518-02-13 00:35] wakes up
fn parse_to_guards(input: Vec<&str>) -> HashMap<usize, Vec<usize>> {
    let mut guards: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut guard_id = 0;
    let mut start = 0;
    let mut finish = 0;

    for line in input {
        let bracket = line.find("]").unwrap();
        let minute = line
            .get(bracket - 2..bracket)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let string = line
            .get(bracket + 2..)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();

        match string[0] {
            "Guard" => guard_id = string[1].get(1..).unwrap().parse::<usize>().unwrap(),
            "falls" => start = minute,
            "wakes" => {
                finish = minute;
                if guards.contains_key(&guard_id) {
                    let guard = guards.get_mut(&guard_id).unwrap();

                    for min in start..finish {
                        guard[min] += 1;
                    }
                } else {
                    let mut minutes: Vec<usize> = vec![0; 60];

                    for min in start..finish {
                        minutes[min] += 1;
                    }

                    guards.insert(guard_id, minutes);
                }
            }
            _ => panic!("Toilet won't flush."),
        }
    }

    guards
}
