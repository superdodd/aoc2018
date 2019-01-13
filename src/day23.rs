use aoc_runner_derive::{aoc, aoc_generator};
use num_traits::abs;
use num_traits::signum;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Nanobot {
    range: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Nanobot {
    #[inline]
    fn is_in_range(&self, x: i32, y: i32, z: i32) -> bool {
        self.distance_to(x, y, z) <= self.range
    }

    #[inline]
    fn distance_to(&self, x: i32, y: i32, z: i32) -> i32 {
        abs(self.x - x) + abs(self.y - y) + abs(self.z - z)
    }
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<Nanobot> {
    let input_re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();

    let mut ret = Vec::new();
    for bot_cap in input_re.captures_iter(input) {
        ret.push(Nanobot {
            range: bot_cap[4].parse::<i32>().unwrap(),
            x: bot_cap[1].parse::<i32>().unwrap(),
            y: bot_cap[2].parse::<i32>().unwrap(),
            z: bot_cap[3].parse::<i32>().unwrap(),
        })
    }

    ret
}

#[derive(PartialEq, Eq, Debug)]
struct Region {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,

    volume: usize,
    min_dist_to_origin: usize,

    lower_bound_bots: Vec<Nanobot>,
    upper_bound_bots: Vec<Nanobot>,
}

fn closest_to_zero(left: i32, right: i32) -> i32 {
    debug_assert!(left <= right);
    if signum(left) != signum(right) {
        0
    } else if left < 0 {
        right
    } else {
        left
    }
}

impl Region {
    fn update_min_dist_to_origin(&mut self) {
        let x = closest_to_zero(self.x_min, self.x_max);
        let y = closest_to_zero(self.y_min, self.y_max);
        let z = closest_to_zero(self.z_min, self.z_max);
        self.min_dist_to_origin = (abs(x) + abs(y) + abs(z)) as usize;
    }

    /// Returns an attempt at a minimum-size rectangular region intersecting all of the bots in the list.
    fn new(bots: &Vec<Nanobot>) -> Region {
        let mut x_min = std::i32::MAX;
        let mut x_max = std::i32::MIN;
        let mut y_min = std::i32::MAX;
        let mut y_max = std::i32::MIN;
        let mut z_min = std::i32::MAX;
        let mut z_max = std::i32::MIN;

        for bot in bots {
            x_min = min(bot.x, x_min);
            x_max = max(bot.x, x_max);
            y_min = min(bot.y, y_min);
            y_max = max(bot.y, y_max);
            z_min = min(bot.z, z_min);
            z_max = max(bot.z, z_max);
        }

        x_min = min(x_min, x_max);
        y_min = min(y_min, y_max);
        z_min = min(z_min, z_max);

        let mut ret = Region {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            volume: ((x_max - x_min + 1) * (y_max - y_min + 1) * (z_max - z_min + 1)) as usize,
            min_dist_to_origin: 0,
            lower_bound_bots: Vec::with_capacity(bots.len()),
            upper_bound_bots: Vec::with_capacity(bots.len()),
        };
        ret.update_min_dist_to_origin();

        // All bots in the list at least intersect this region.
        for b in bots {
            if ret.covered_by_bot(b) {
                ret.lower_bound_bots.push(*b);
            } else {
                ret.upper_bound_bots.push(*b);
            }
        }

        ret
    }

    /// A bot intersects this region if:
    ///   - The bot's location is within the bounds of the region on two axes, and within the
    ///     bounds extended by the bot's range of the third axis, or
    ///   - The bot is within range of any corner of the region.
    fn bot_intersects(&self, bot: &Nanobot) -> bool {
        let xr = bot.x >= self.x_min && bot.x <= self.x_max;
        let xe = bot.x >= self.x_min - bot.range && bot.x <= self.x_max + bot.range;
        let yr = bot.y >= self.y_min && bot.y <= self.y_max;
        let ye = bot.y >= self.y_min - bot.range && bot.y <= self.y_max + bot.range;
        let zr = bot.z >= self.z_min && bot.z <= self.z_max;
        let ze = bot.z >= self.z_min - bot.range && bot.z <= self.z_max + bot.range;

        // If the bot is outside the range-extended region bounds on all axes, it definitely
        // does not intersect the region.
        if !(xe && ye && ze) {
            return false;
        }

        // Otherwise, if the bot is within the region bounds along two axes, check to see if it's
        // within the range-extended bounds along the third axis.
        if xr && yr {
            return ze;
        }

        if xr && zr {
            return ye;
        }

        if yr && zr {
            return xe;
        }

        // Finally, check if the bot is otherwise in range of any corner of the region.
        bot.is_in_range(self.x_min, self.y_min, self.z_min)
            || bot.is_in_range(self.x_min, self.y_min, self.z_max)
            || bot.is_in_range(self.x_min, self.y_max, self.z_min)
            || bot.is_in_range(self.x_min, self.y_max, self.z_max)
            || bot.is_in_range(self.x_max, self.y_min, self.z_min)
            || bot.is_in_range(self.x_max, self.y_min, self.z_max)
            || bot.is_in_range(self.x_max, self.y_max, self.z_min)
            || bot.is_in_range(self.x_max, self.y_max, self.z_max)
    }

    /// A bot covers this region if every corner of the region is in range of the bot.
    fn covered_by_bot(&self, bot: &Nanobot) -> bool {
        bot.distance_to(self.x_min, self.y_min, self.z_min) <= bot.range
            && bot.distance_to(self.x_min, self.y_min, self.z_max) <= bot.range
            && bot.distance_to(self.x_min, self.y_max, self.z_min) <= bot.range
            && bot.distance_to(self.x_min, self.y_max, self.z_max) <= bot.range
            && bot.distance_to(self.x_max, self.y_min, self.z_min) <= bot.range
            && bot.distance_to(self.x_max, self.y_min, self.z_max) <= bot.range
            && bot.distance_to(self.x_max, self.y_max, self.z_min) <= bot.range
            && bot.distance_to(self.x_max, self.y_max, self.z_max) <= bot.range
    }

    fn split_region(&self) -> Vec<Region> {
        let xmid: i32 = (self.x_min + self.x_max) / 2;
        let ymid: i32 = (self.y_min + self.y_max) / 2;
        let zmid: i32 = (self.z_min + self.z_max) / 2;
        let xa = (self.x_min, xmid);
        let xb = (xmid + 1, self.x_max);
        let ya = (self.y_min, ymid);
        let yb = (ymid + 1, self.y_max);
        let za = (self.z_min, zmid);
        let zb = (zmid + 1, self.z_max);
        [
            (xa, ya, za),
            (xa, ya, zb),
            (xa, yb, za),
            (xa, yb, zb),
            (xb, ya, za),
            (xb, ya, zb),
            (xb, yb, za),
            (xb, yb, zb),
        ]
        .iter()
        .filter(|(xr, yr, zr)| {
            xr.0 <= xr.1
                && yr.0 <= yr.1
                && zr.0 <= zr.1
                && (xr.0, xr.1, yr.0, yr.1, zr.0, zr.1)
                    != (
                        self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max,
                    )
        })
        .map(|(xr, yr, zr)| {
            let mut ret = Region {
                x_min: xr.0,
                x_max: xr.1,
                y_min: yr.0,
                y_max: yr.1,
                z_min: zr.0,
                z_max: zr.1,
                volume: ((xr.1 - xr.0 + 1) * (yr.1 - yr.0 + 1) * (zr.1 - zr.0 + 1)) as usize,
                // All bots that are in the lower bound of the pre-split region are also in
                // the lower bound of the post-split region.  Some bots may move from the upper
                // bound list to the lower bound list, or out of the upper bound list entirely.
                // All bots that are in the lower bound list of the post-split region are in
                // either the upper or lower bound list of the pre-split region.
                min_dist_to_origin: 0,
                lower_bound_bots: self.lower_bound_bots.to_vec(),
                // If a bot is in the post-split upper bound list then it was also in the
                // pre-split upper bound list.
                upper_bound_bots: Vec::with_capacity(self.upper_bound_bots.len()),
            };
            ret.update_min_dist_to_origin();
            for b in &self.upper_bound_bots {
                if ret.covered_by_bot(b) {
                    ret.lower_bound_bots.push(*b);
                } else if ret.bot_intersects(b) {
                    ret.upper_bound_bots.push(*b);
                }
            }

            ret
        })
        .into_iter()
        .collect::<Vec<Region>>()
    }
}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prefer these attributes, in decreasing order of preference:
        // - Lower bound + upper bound lengths (greater preferred)
        // - Lower bound length (greater preferred)
        // - smallest distance to origin for any point in region (smaller preferred)
        // - Size (smaller volume preferred)
        // - min corner (smaller preferred)
        // - max corner (smaller preferred)
        (self.lower_bound_bots.len() + self.upper_bound_bots.len())
            .cmp(&(other.lower_bound_bots.len() + other.upper_bound_bots.len()))
            .then(
                self.lower_bound_bots
                    .len()
                    .cmp(&other.lower_bound_bots.len()),
            )
            .then(other.min_dist_to_origin.cmp(&self.min_dist_to_origin))
            .then(other.volume.cmp(&self.volume))
            .then(other.x_min.cmp(&self.x_min))
            .then(other.y_min.cmp(&self.y_min))
            .then(other.z_min.cmp(&self.z_min))
            .then(other.x_max.cmp(&self.x_max))
            .then(other.y_max.cmp(&self.y_max))
            .then(other.z_max.cmp(&self.z_max))
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day23, part1)]
fn solve_part1(bots: &Vec<Nanobot>) -> usize {
    let ref_bot = bots.iter().max_by(|&a, &b| a.range.cmp(&b.range)).unwrap();
    println!("Max range bot: {:?}", ref_bot);

    bots.iter()
        .filter(|&b| ref_bot.is_in_range(b.x, b.y, b.z))
        .count()
}

#[aoc(day23, part2)]
fn solve_part2(bots: &Vec<Nanobot>) -> usize {
    // Recursively split the total search space, determining the upper and lower bounds for each
    // subdivision.  We can discard subdivisions whose upper bound is smaller than the largest
    // lower bound we've found for any other region.
    let mut best_region: Option<Region> = None;

    let mut region_heap = BinaryHeap::new();
    let start_region = Region::new(bots);
    //println!("{:?}", start_region);
    region_heap.push(start_region);
    let mut loop_count = 0usize;
    while let Some(region) = region_heap.pop() {
        loop_count += 1;
        if region.upper_bound_bots.is_empty() {
            // No need to further split this region; see if it's better than the
            // previous best we've found so far.
            best_region = match best_region.as_ref().map(|b| b.cmp(&region)) {
                None | Some(Ordering::Less) => {
                    println!(
                        "Loop {}: Found better region: count={}, dist={}",
                        loop_count,
                        region.lower_bound_bots.len(),
                        region.min_dist_to_origin
                    );
                    Some(region)
                }
                _ => best_region,
            };
            continue;
        }

        for s in region.split_region() {
            match best_region.as_ref() {
                // If this split of the region has no chance of being better than the currently-known best, discard it.
                Some(b)
                    if s.lower_bound_bots.len() + s.upper_bound_bots.len()
                        < b.lower_bound_bots.len() =>
                {
                    ()
                }
                Some(b)
                    if s.lower_bound_bots.len() + s.upper_bound_bots.len()
                        == b.lower_bound_bots.len()
                        && s.min_dist_to_origin >= b.min_dist_to_origin =>
                {
                    ()
                }
                // Otherwise, put it back on the pile for further examination.
                _ => region_heap.push(s),
            }
        }
    }

    if let Some(r) = best_region {
        println!(
            "Loop {}: Best region found. count={}, dist={}, vol={}",
            loop_count,
            r.lower_bound_bots.len(),
            r.min_dist_to_origin,
            r.volume
        );
        return r.min_dist_to_origin;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "pos=<0,0,0>, r=4
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
        let input = "pos=<0,0,0>, r=4
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
    fn test_split_region() {
        let bot1 = Nanobot {
            range: 5,
            x: 1,
            y: 2,
            z: 2,
        };
        let bot2 = Nanobot {
            range: 3,
            x: 3,
            y: 3,
            z: 3,
        };
        let test_region = Region {
            x_min: 0,
            x_max: 3,
            y_min: 0,
            y_max: 3,
            z_min: 0,
            z_max: 3,
            volume: 4 * 4 * 4,
            min_dist_to_origin: 0,
            lower_bound_bots: vec![bot1],
            upper_bound_bots: vec![bot2],
        };
        assert_eq!(
            test_region.split_region(),
            vec![
                (0, 1, 0, 1, 0, 1, vec![bot1], vec![]),
                (0, 1, 0, 1, 2, 3, vec![bot1], vec![]),
                (0, 1, 2, 3, 0, 1, vec![bot1], vec![]),
                (0, 1, 2, 3, 2, 3, vec![bot1], vec![bot2]),
                (2, 3, 0, 1, 0, 1, vec![bot1], vec![]),
                (2, 3, 0, 1, 2, 3, vec![bot1], vec![bot2]),
                (2, 3, 2, 3, 0, 1, vec![bot1], vec![bot2]),
                (2, 3, 2, 3, 2, 3, vec![bot1, bot2], vec![])
            ]
            .into_iter()
            .map(
                |(x_min, x_max, y_min, y_max, z_min, z_max, lower_bound_bots, upper_bound_bots)| {
                    let mut ret = Region {
                        x_min,
                        x_max,
                        y_min,
                        y_max,
                        z_min,
                        z_max,
                        volume: ((x_max - x_min + 1) * (y_max - y_min + 1) * (z_max - z_min + 1))
                            as usize,
                        min_dist_to_origin: 0,
                        lower_bound_bots,
                        upper_bound_bots,
                    };
                    ret.update_min_dist_to_origin();
                    ret
                }
            )
            .collect::<Vec<Region>>()
        );
    }

    #[test]
    fn test_solve_part2() {
        let input = "pos=<10,12,12>, r=2
                            pos=<12,14,12>, r=2
                            pos=<16,12,12>, r=4
                            pos=<14,14,14>, r=6
                            pos=<50,50,50>, r=200
                            pos=<10,10,10>, r=5";

        assert_eq!(36, solve_part2(&parse_input(input)));
    }

}
