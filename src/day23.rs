use aoc_runner_derive::{aoc,aoc_generator};
use num_traits::abs;
use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Nanobot {
    range: usize,
    x: i64,
    y: i64,
    z: i64,
}

impl Nanobot {

    fn is_in_range(&self, other: &Nanobot) -> bool {
        self.distance_to(other) <= self.range
    }

    fn distance_to(&self, other: &Nanobot) -> usize {
        (abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)) as usize
    }
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<Nanobot> {
    let input_re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();

    let mut ret = Vec::new();
    for bot_cap in input_re.captures_iter(input) {
        ret.push(Nanobot {
            range: bot_cap[4].parse::<usize>().unwrap(),
            x: bot_cap[1].parse::<i64>().unwrap(),
            y: bot_cap[2].parse::<i64>().unwrap(),
            z: bot_cap[3].parse::<i64>().unwrap(),
        })
    }

    ret
}

#[aoc(day23, part1)]
fn solve_part1(bots: &Vec<Nanobot>) -> usize {
    let ref_bot = bots.iter().max_by(|&a, &b| a.range.cmp(&b.range)).unwrap();
    println!("Max range bot: {:?}", ref_bot);

    bots.iter().filter(|&b| ref_bot.is_in_range(b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input =  "pos=<0,0,0>, r=4
                            pos=<1,0,0>, r=1
                            pos=<4,0,0>, r=3
                            pos=<0,2,0>, r=1
                            pos=<0,5,0>, r=3
                            pos=<0,0,3>, r=1
                            pos=<1,1,1>, r=1
                            pos=<1,1,2>, r=1
                            pos=<-1,-1,2>, r=1
                            pos=<1,3,1>, r=1";
        assert_eq!(10, parse_input(input).len());
    }

    #[test]
    fn test_solve_part1() {
        let input =  "pos=<0,0,0>, r=4
                            pos=<1,0,0>, r=1
                            pos=<4,0,0>, r=3
                            pos=<0,2,0>, r=1
                            pos=<0,5,0>, r=3
                            pos=<0,0,3>, r=1
                            pos=<1,1,1>, r=1
                            pos=<1,1,2>, r=1
                            pos=<-1,-1,2>, r=1
                            pos=<1,3,1>, r=1";
        let bots = parse_input(input);
        assert_eq!(8, solve_part1(&bots));
    }
}