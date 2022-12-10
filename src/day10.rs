use aoc_runner_derive::{aoc, aoc_generator};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Instructions {
    Noop,
    Addx(i64),
}

impl FromStr for Instructions {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instructions::Noop)
        } else {
            let num = s.split_once(' ').unwrap().1.parse().unwrap();
            Ok(Instructions::Addx(num))
        }
    }
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Instructions> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[Instructions]) -> i64 {
    let mut x = 1;
    let mut cycle = 1;
    let mut total = 0;
    for ins in inputs {
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            total += cycle * x;
        }

        match ins {
            Instructions::Noop => {
                cycle += 1;
            }
            Instructions::Addx(n) => {
                cycle += 1;
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    total += cycle * x;
                }
                x += n;
                cycle += 1;
            }
        }
    }
    total
}

fn draw(screen: &mut [[u8; 40]; 6], cycle: i64, x: i64) {
    let cycle = cycle - 1;
    let row = cycle / 40;
    let col = cycle % 40;
    if col.abs_diff(x) <= 1 {
        screen[row as usize][col as usize] = b'#';
    }
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Instructions]) -> String {
    let mut screen = [[b'.'; 40]; 6];

    let mut x = 1;
    let mut cycle = 1;
    for ins in inputs {
        draw(&mut screen, cycle, x);
        match ins {
            Instructions::Noop => {
                cycle += 1;
            }
            Instructions::Addx(n) => {
                cycle += 1;
                draw(&mut screen, cycle, x);
                x += n;
                cycle += 1;
            }
        }
    }

    unsafe {
        String::from_utf8_unchecked(screen.into_iter().fold(
            Vec::with_capacity(41 * 6),
            |mut acc, line| {
                acc.push(b'\n');
                acc.extend(line.into_iter());
                acc
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("day10/SAMPLE2.txt");

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 13140);
    }

    #[test]
    pub fn test2() {
        const ANS: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

        assert_eq!(part2(&generator(SAMPLE)), ANS);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2022/day10.txt");
        const ANSWERS: (i64, &str) = (
            13680,
            r#"
###..####..##..###..#..#.###..####.###..
#..#....#.#..#.#..#.#.#..#..#.#....#..#.
#..#...#..#....#..#.##...#..#.###..###..
###...#...#.##.###..#.#..###..#....#..#.
#....#....#..#.#....#.#..#....#....#..#.
#....####..###.#....#..#.#....####.###.."#,
        );

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}