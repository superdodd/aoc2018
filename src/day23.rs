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

/// Returns an upper bound on the largest number of bots reachable by any point within the described
/// cubic region.  Axis ranges are lower-bound inclusive, upper-bound exclusive.
/// This is the number of bots that are in range of any point within the region.
fn upper_bound_for_region(bots: &Vec<Nanobot>, x: (usize, usize), y: (usize, usize), z: (usize, usize)) -> usize {
    // A bot has points in range if:
    // - The bot's location is contained within the region, or
    // - The bot is within range of any corner of the region, or
    // - The bot is in between two planes created by the region along any pair of axes,
    //     and is within the bot's range of the endpoints of the region along the third axis.
}

/// Returns a lower bound on the largest number of bots reachable by any point within the described
/// cubic region.  Axis ranges are lower-bound inclusive, upper-bound exclusive.
/// This is the number of bots for which *all* points in the region are in range.
fn lower_bound_for_region(bots: &Vec<Nanobot>, x: (usize, usize), y: (usize, usize), z: (usize, usize)) -> usize {
    // Note that a bot overlaps a region entirely iff the corners of the region are all in range.

}

/// Splits a region into eight sub-regions, subdividing each axis range in half.
fn split_region(x: (usize, usize), y: (usize, usize), z: (usize, usize)) -> [((usize, usize), (usize, usize), (usize, usize)); 8] {
    let xmid = (x.0 + x.1) / 2;
    let ymid = (y.0 + y.1) / 2;
    let zmid = (z.0 + z.1) / 2;
    let xa = (x.0, xmid);
    let xb = (xmid, x.1);
    let ya = (y.0, ymid);
    let yb = (ymid, y.1);
    let za = (z.0, zmid);
    let zb = (zmid, z.0);
    [
        (xa, ya, za), (xa, ya, zb), (xa, yb, za), (xa, yb, zb),
        (xb, ya, za), (xb, ya, zb), (xb, yb, za), (xb, yb, zb),
    ]
}

#[aoc(day23, part1)]
fn solve_part1(bots: &Vec<Nanobot>) -> usize {
    let ref_bot = bots.iter().max_by(|&a, &b| a.range.cmp(&b.range)).unwrap();
    println!("Max range bot: {:?}", ref_bot);

    bots.iter().filter(|&b| ref_bot.is_in_range(b)).count()
}

#[aoc(day23, part2)]
fn solve_part2(bots: &Vec<Nanobot>) -> usize {
    // Recursively split the total search space, determining the upper and lower bounds for each
    // subdivision.  We can discard subdivisions whose upper bound is smaller than the largest
    // lower bound we've found for any other region.
    // Order our search by upper bound (always further subdivide regions with higher upper bounds first).
    unimplemented!()
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

    #[test]
    fn test_upper_bound_for_region() {
        unimplemented!()
    }
}