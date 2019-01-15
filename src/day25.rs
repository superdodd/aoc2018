use aoc_runner_derive::aoc_generator;

use num_traits::abs;
use std::cmp::Ordering;
use std::cmp::min;
use std::cmp::max;
use std::ops::BitAnd;

use itertools::Itertools;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Point([i32; 4]);

impl Point {
    fn new(x: i32, y: i32, z: i32, t: i32) -> Point {
        Point([x, y, z, t])
    }

    fn distance_to(&self, other: &Self) -> usize {
        (0..4).map(|i| abs(self.0[i] - other.0[i]) as usize).sum()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        for axis in 0..4 {
            match self.0[axis].cmp(&other.0[axis]) {
                Ordering::Equal => continue,
                c @ _ => return c,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Clone, Debug)]
struct Constellation {
    // The minimal axis-aligned region containing this constellation.
    bounds: BoundingBox,
    points: Vec<Point>,
}

impl Constellation {
    fn new(point: &Point) -> Constellation {
        Constellation{
            bounds: BoundingBox{
                min: point.clone(),
                max: point.clone(),
            },
            points: vec![point.clone()]
        }
    }

    fn check_add(&mut self, point: &Point) -> bool {
        // If we're farther than 3 from the bounding box, no need to check anything else.
        if self.bounds.distance_to_point(point) > 3 {
            return false;
        }
        match self.points.iter().find(|&p| p.distance_to(point) <= 3) {
            None => false,
            Some(o) => {
                self.points.push(point.clone());
                self.points.sort();
                self.bounds.expand_to_point(point);
                true
            }
        }
    }

    // Non-destructively merge two Constellations together, returning the merged result.
    fn merge(&self, other: &Constellation) -> Constellation {
        Constellation {
            bounds: self.bounds.merge(&other.bounds),
            points: self.points.iter().merge(other.points.iter()).dedup().map(|p| p.clone()).collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {

    fn corners(&self) -> Vec<Point> {
        let mut ret = Vec::with_capacity(16);
        for i in 0..16 {
            let mut coords = Point { 0: [0; 4] };
            for axis in 0..4 {
                coords.0[i] = if i.bitand(1 << i) == 1 { self.min.0[i] } else { self.max.0[i] };
            }
            ret.push(coords)
        }
        ret
    }

    fn distance_to_point(&self, point: &Point) -> usize {
        // The distance from a point to a bounding box is the sum of the single-axis
        // distances from the point's coordinate on that axis to the range covered by
        // the bounding box on that axis.
        // If the point is completely contained in the bounding box, returns 0.
        let mut ret: usize = 0;
        for d in 0..4 {
            if point.0[d] < self.min.0[d] {
                ret += (self.min.0[d] - point.0[d]) as usize;
            } else if point.0[d] > self.max.0[d] {
                ret += (point.0[d] - self.max.0[d]) as usize;
            }
        }
        ret
    }

    fn expand_to_point(&mut self, point: &Point) {
        for i in 0..4 {
            self.min.0[i] = min(self.min.0[i], point.0[i]);
            self.max.0[i] = max(self.max.0[i], point.0[i]);
        }
    }

    fn merge(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        for p in other.corners() {
            ret.expand_to_point(&p);
        }
        ret
    }

}

#[aoc_generator(day25, part1)]
fn parse_input(input: &str) -> Vec<Constellation> {
    let mut ret: Vec<Constellation> = Vec::new();
    for line in input.lines() {
        let mut coords = [0i32; 4];
        let parsed_coords = line.split(",").map(|i: &str| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        coords.copy_from_slice(&parsed_coords[0..4]);
        let point = Point(coords);

        let mut found_constellation: Option<Constellation> = None;

        let mut new_ret: Vec<Constellation> = Vec::with_capacity(ret.len() + 1);
        for c in ret.into_iter().as_mut_slice() {
            // If this point is part of the constellation, then see if we need to merge
            // the constellation with any others that the point may also be part of.
            // Hold off on adding the merged/expanded constellation to the list until
            // we've examined all of the other constellations.
            if c.check_add(&point) {
                if let Some(f) = found_constellation.take() {
                    found_constellation = Some(f.merge(c));
                } else {
                    found_constellation = Some(c.clone());
                }
            } else {
                // If the point isn't part of this constellation, just add the constellation
                // to the updated list as-is.
                new_ret.push(c.clone());
            }
        }
        // If we found a matching constellation (or more) that this point belongs to,
        // make sure the merged constellation is in the final list.  Otherwise,
        // stick a new single-point constellation in the list.
        if let Some(f) = found_constellation.take() {
            new_ret.push(f);
        } else {
            new_ret.push(Constellation::new(&point));
        }

        ret = new_ret;
    }

    ret
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let input =
           "-1,2,2,0
            0,0,2,-2
            0,0,0,-2
            -1,2,0,0
            -2,-2,-2,2
            3,0,2,-1
            -1,3,2,2
            -1,0,-1,0
            0,2,1,-2
            3,0,0,0";

        let constellations = parse_input(input);
        assert_eq!(4, constellations.len());
    }
}