use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Clone)]
struct MapState {
    m: Vec<Vec<char>>,
    xmin: usize,
    xrange: usize,
    ymin: usize,
    yrange: usize,
    source: usize,
}

impl MapState {
    fn simulate(&mut self) {
        // Start processing at the source block
        let mut update_queue: Vec<(usize, usize)> = vec![(self.source, 0)];

        while let Some((x, y)) = update_queue.pop() {
            //println!("Updating {:?} - {}", (x, y), update_queue.len());
            match self.at(x, y) {
                '+' | '|' => {
                    if self.can_fall(x, y) {
                        if y + 1 < self.yrange && self.at(x, y + 1) == '.' {
                            update_queue.push((x, y + 1));
                            self.m[y + 1][x] = '|';
                        }
                    } else {
                        // Fill the contained row if possible
                        let mut fill_is_still = false;
                        // Check left...
                        let mut xleft = x - 1;
                        loop {
                            let c = self.at(xleft, y);
                            if c == '|' {
                                // If we encounter any falling water, keep looking for walls, but
                                // see if we need to update the row above as well
                                update_queue.push((xleft, y - 1));
                            }
                            if c == '#' {
                                fill_is_still = true;
                                xleft += 1;
                                break;
                            }
                            if self.can_fall(xleft, y) {
                                break;
                            }
                            xleft -= 1;
                            if xleft == 0 {
                                break;
                            }
                        }
                        // And right
                        let mut xright = x + 1;
                        while xright < self.xrange {
                            let c = self.at(xright, y);
                            if c == '|' {
                                // If we encounter any falling water, keep looking for walls, but
                                // see if we need to update the row above as well
                                update_queue.push((xright, y - 1));
                            }
                            if c == '#' {
                                xright -= 1;
                                break;
                            }
                            if self.can_fall(xright, y) {
                                fill_is_still = false;
                                break;
                            }
                            xright += 1;
                        }
                        //println!("Filling ({}..{},{}) ({})", xleft, xright, y, fill_is_still);
                        for i in xleft..=xright {
                            self.m[y][i] = if fill_is_still { '~' } else { '|' };
                        }
                        if fill_is_still {
                            // Need to check the next row up to see if we also need to fill
                            update_queue.push((x, y - 1));
                        } else {
                            // Check the ends to see if either one should continue falling
                            if self.can_fall(xleft, y) {
                                update_queue.push((xleft, y));
                            }
                            if self.can_fall(xright, y) {
                                update_queue.push((xright, y));
                            }
                        }
                    }
                }
                '#' | '.' | '~' => (), // Nothing to do
                c => {
                    println!("{}", self);
                    panic!("Invalid map state {} at ({}, {})", c, x, y)
                }
            }
            //println!("{:?}", update_queue);
            update_queue.sort();
            update_queue.dedup();
        }
    }

    fn can_fall(&self, x: usize, y: usize) -> bool {
        self.get(x, y + 1)
            .map(|c| c == '.' || c == '|')
            .unwrap_or(true)
    }

    fn at(&self, x: usize, y: usize) -> char {
        self.get(x, y).unwrap()
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.m.get(y) {
            if let Some(c) = row.get(x) {
                return Some(*c);
            }
        }
        None
    }

    fn score_parts(&self) -> (usize, usize) {
        let mut still_score = 0;
        let mut flow_score = 0;
        let ystart = if self.ymin == 0 { 1 } else { 2 };
        for y in ystart..self.yrange {
            for x in 0..self.xrange {
                let c = self.at(x, y);
                if c == '|' {
                    flow_score += 1;
                } else if c == '~' {
                    still_score += 1;
                }
            }
        }
        (still_score, flow_score)
    }

    fn score(&self) -> usize {
        let (s, f) = self.score_parts();
        s + f
    }

    fn still_score(&self) -> usize {
        self.score_parts().0
    }
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(
            f,
            "x=({},{}) y=({},{})",
            self.xmin,
            self.xmin + self.xrange,
            self.ymin,
            self.ymin + self.yrange
        )?;
        for l in &self.m {
            writeln!(f, "{}", l.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> MapState {
    // Build a vector of ranges first from the input
    let range_regex = Regex::new(r"(?:(x)|(y))=(\d+), [xy]=(\d+)..(\d+)").unwrap();
    let mut ranges: Vec<(usize, usize, usize, usize)> = Vec::new();
    for c in range_regex.captures_iter(input) {
        let nums = &vec![
            c[3].parse::<usize>().unwrap(),
            c[4].parse::<usize>().unwrap(),
            c[5].parse::<usize>().unwrap(),
        ];
        let r = match (c.get(1), c.get(2)) {
            (Some(_x), None) => (nums[0], nums[0], nums[1], nums[2]),
            (None, Some(_y)) => (nums[1], nums[2], nums[0], nums[0]),
            (errx, erry) => panic!("Unexpected capture group contents {:?},{:?}", errx, erry),
        };
        ranges.push(r);
    }

    let limits: (usize, usize, usize, usize) = ranges.iter().fold(
        (
            std::usize::MAX,
            std::usize::MIN,
            std::usize::MAX,
            std::usize::MIN,
        ),
        |acc, (xmin, xmax, ymin, ymax)| {
            (
                min(acc.0, *xmin),
                max(acc.1, *xmax),
                min(acc.2, *ymin),
                max(acc.3, *ymax),
            )
        },
    );

    let xmin = limits.0 - 1;
    let source = 500 - xmin;
    let xrange = max(limits.1, 500) - xmin + 2;
    let ymin = max(limits.2, 2) - 2;
    let yrange = limits.3 - ymin + 1;

    let mut m: Vec<Vec<char>> = vec![vec!['.'; xrange]; yrange];
    m[0][source] = '+';

    for r in ranges {
        for y in r.2..=r.3 {
            for x in r.0..=r.1 {
                m[y - ymin][x - xmin] = '#';
            }
        }
    }
    MapState {
        m,
        xmin,
        xrange,
        ymin,
        yrange,
        source,
    }
}

#[aoc(day17, part1)]
fn solve_part1(input: &MapState) -> usize {
    let mut map = input.clone();
    map.simulate();
    println!("{}", map);
    map.score()
}

#[aoc(day17, part2)]
fn solve_part2(input: &MapState) -> usize {
    let mut map = input.clone();
    map.simulate();
    map.still_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> MapState {
        let test_parse_input = "x=495, y=2..7
            y=7, x=495..501
            x=501, y=3..7
            x=498, y=2..4
            x=506, y=1..2
            x=498, y=10..13
            x=504, y=10..13
            y=13, x=498..504";
        let map = parse(test_parse_input);
        println!("{}", map);
        map
    }

    #[test]
    fn test_parse() {
        let map = get_test_input();
        assert_eq!(map.m[0][6], '+');
    }

    #[test]
    fn test_solve_part1() {
        let map = get_test_input();
        assert_eq!(57, solve_part1(&map));
    }

    fn get_loop_input() -> MapState {
        let test_parse_input = "x=495, y=5..10
            x=505, y=5..10
            y=10, x=495..505
            x=500, y=7..8";
        let map = parse(test_parse_input);
        println!("{}", map);
        map
    }

    #[test]
    fn test_loop() {
        let mut map = get_loop_input();
        map.simulate();
        println!("{}", map);
    }
}
