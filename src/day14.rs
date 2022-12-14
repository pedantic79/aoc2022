use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;

use crate::common::utils::parse_pair;

#[aoc_generator(day14)]
pub fn generator(input: &str) -> HashMap<(usize, usize), u8> {
    let mut map = HashMap::default();
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|e| parse_pair::<usize>(e).unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|line| {
            line.windows(2).for_each(|x| {
                if x[0].0 == x[1].0 {
                    for y in x[0].1..=x[1].1 {
                        map.insert((x[0].0, y), b'#');
                    }
                    for y in x[1].1..=x[0].1 {
                        map.insert((x[0].0, y), b'#');
                    }
                } else {
                    for y in x[0].0..=x[1].0 {
                        map.insert((y, x[0].1), b'#');
                    }
                    for y in x[1].0..=x[0].0 {
                        map.insert((y, x[0].1), b'#');
                    }
                }
            })
        });

    map
}

#[aoc(day14, part1)]
pub fn part1(map: &HashMap<(usize, usize), u8>) -> usize {
    let max = map.keys().map(|x| x.1).max().unwrap();
    let mut map = map.clone();

    let mut count = 0;

    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y > max {
                break 'outer;
            }

            if !map.contains_key(&(x, y + 1)) {
                y += 1;
                continue;
            } else if !map.contains_key(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            } else if !map.contains_key(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            } else {
                break;
            }
        }
        map.insert((x, y), b'o');
        count += 1;
    }

    count
}

#[aoc(day14, part2)]
pub fn part2(map: &HashMap<(usize, usize), u8>) -> usize {
    let max = map.keys().map(|x| x.1).max().unwrap();
    let mut map = map.clone();
    for x in 0..1000 {
        map.insert((x, max + 2), b'#');
    }

    let mut count = 0;
    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if !map.contains_key(&(x, y + 1)) {
                y += 1;
                continue;
            } else if !map.contains_key(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            } else if !map.contains_key(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            } else if map.get(&(x, y)) == Some(&b'o') && y == 0 {
                break 'outer;
            }
            map.insert((x, y), b'o');
            count += 1;
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 24);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 93);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day14.txt");
        const ANSWERS: (usize, usize) = (644, 27324);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
