use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::min;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq)]
struct MapState {
    m: Vec<Vec<char>>,
    xmax: usize,
    ymax: usize,
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> MapState {
    let mut m: Vec<Vec<char>> = Vec::new();

    let mut row: Vec<char> = Vec::new();
    for c in input.chars() {
        if c == ' ' {
            continue;
        }
        if c == '\n' {
            if !row.is_empty() {
                m.push(row);
            }
            row = Vec::new();
        } else {
            row.push(c);
        }
    }
    if !row.is_empty() {
        m.push(row);
    }
    let ymax = m.len();
    assert!(m.iter().all(|i| i.len() == ymax));
    let xmax = m[0].len();
    MapState { m, xmax, ymax }
}

#[aoc(day18, part1)]
fn solve_part1(input: &MapState) -> usize {
    let mut m = input.clone();
    for _ in 1..=10 {
        m.step_time();
    }
    m.score()
}

#[aoc(day18, part2)]
fn solve_part2(input: &MapState) -> usize {
    let mut fast = input.clone();
    fast.step_time();
    fast.step_time();
    let mut slow = input.clone();
    slow.step_time();
    let mut slow_gen: u32 = 1;
    let target_gen: u32 = 1_000_000_000;
    while fast != slow {
        slow_gen += 1;
        fast.step_time();
        fast.step_time();
        slow.step_time();
    }
    let gen = (target_gen / slow_gen) * slow_gen;
    for _g in gen..target_gen {
        fast.step_time();
    }
    fast.score()
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in &self.m {
            writeln!(f, "{}", row.iter().collect::<String>())?
        }
        Ok(())
    }
}

impl MapState {
    fn get_neighbors(&self, x: usize, y: usize) -> (usize, usize, usize) {
        let mut tree: usize = 0;
        let mut lumb: usize = 0;
        let mut open: usize = 0;
        for j in y.checked_sub(1).unwrap_or(0)..=min(y + 1, self.ymax - 1) {
            for i in x.checked_sub(1).unwrap_or(0)..=min(x + 1, self.xmax - 1) {
                if (x, y) == (i, j) {
                    continue;
                }
                match self.m[j][i] {
                    '|' => tree += 1,
                    '#' => lumb += 1,
                    '.' => open += 1,
                    c => panic!("Invalid map at ({}, {}): {}", i, j, c),
                }
            }
        }
        (tree, lumb, open)
    }

    fn step_time(&mut self) {
        let mut new_map: Vec<Vec<char>> = Vec::new();
        let mut row: Vec<char> = Vec::new();
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                let (tree, lumb, _open) = self.get_neighbors(x, y);
                let c = match self.m[y][x] {
                    // An open acre will become filled with trees if three or more adjacent
                    // acres contained trees. Otherwise, nothing happens.
                    '.' if tree >= 3 => '|',
                    // An acre filled with trees will become a lumberyard if three or more
                    // adjacent acres were lumberyards. Otherwise, nothing happens.
                    '|' if lumb >= 3 => '#',
                    // An acre containing a lumberyard will remain a lumberyard if it was adjacent
                    // to at least one other lumberyard and at least one acre containing trees.
                    // Otherwise, it becomes open.
                    '#' if lumb == 0 || tree == 0 => '.',
                    // ... Otherwise, nothing changes...
                    c => c,
                };
                row.push(c);
            }
            new_map.push(row);
            row = Vec::new();
        }
        self.m = new_map;
    }

    fn score(&self) -> usize {
        let mut tree: usize = 0;
        let mut lumb: usize = 0;
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                match self.m[y][x] {
                    '#' => lumb += 1,
                    '|' => tree += 1,
                    _ => (),
                }
            }
        }
        tree * lumb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> MapState {
        let test_input = "
           .#.#...|#.
           .....#|##|
           .|..|...#.
           ..|#.....#
           #.#|||#|#|
           ...#.||...
           .|....|...
           ||...#|.#|
           |.||||..|.
           ...#.|..|.";

        let m = parse_input(test_input);
        println!("{}", m);
        m
    }

    #[test]
    fn test_parse_input() {
        let _m = get_test_input();
    }

    #[test]
    fn test_step_time() {
        let mut m = get_test_input();
        m.step_time();
        let end = format!("{}", m);
        let expected_input_1 = ".......##.
             ......|###
             .|..|...#.
             ..|#||...#
             ..##||.|#|
             ...#||||..
             ||...|||..
             |||||.||.|
             ||||||||||
             ....||..|.";
        let expected_map_1 = parse_input(expected_input_1);
        assert_eq!(format!("{}", expected_map_1), end);

        let expected_input_10 = ".||##.....
             ||###.....
             ||##......
             |##.....##
             |##.....##
             |##....##|
             ||##.####|
             ||#####|||
             ||||#|||||
             ||||||||||";
        let expected_map_10 = parse_input(expected_input_10);
        for _ in 2..=10 {
            m.step_time();
        }
        assert_eq!(format!("{}", expected_map_10), format!("{}", m));
        assert_eq!(1147, m.score());
    }
}
