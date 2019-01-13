use aoc_runner_derive::aoc;
use num_traits::abs;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

const DEPTH: usize = 11820;
const TARGET: (usize, usize) = (7, 782);

type Equipment = usize;

// This needs to be 1 because we want equipment type to be incompatible only
// with the terrain type of the matching index (since wet=1, torch must be 1)
const EQP_TORCH: Equipment = 1;

type Terrain = usize;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Loc {
    x: usize,
    y: usize,
    e: Equipment,

    target_distance: usize,
    target: (usize, usize),

    erosion_level: usize,
    terrain_type: Terrain,

    cost: usize,
    step_back: (usize, usize, Equipment),
}

impl Ord for Loc {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then(
                (other.x + other.y + other.target_distance)
                    .cmp(&(self.x + self.y + self.target_distance)),
            )
            .then(other.cost.cmp(&self.cost))
            //.then((other.cost + other.target_distance).cmp(&(self.cost + self.target_distance)))
            //.then(self.cost.cmp(&other.cost))
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x))
            .then((self.e as usize).cmp(&(other.e as usize)))
            .then(self.step_back.cmp(&other.step_back))
            .then(self.erosion_level.cmp(&other.erosion_level))
            .then((self.terrain_type as usize).cmp(&(other.terrain_type as usize)))
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Vec<Loc>>>,
    depth: usize,
    target: (usize, usize),

    to_check: BinaryHeap<Loc>,
    shortest_time: Option<usize>,
}

impl Map {
    fn next_steps(&self, loc: &Loc) -> Vec<Loc> {
        let mut ret = Vec::with_capacity(6);
        let step_back = (loc.x, loc.y, loc.e);

        for eq in 0usize..3 {
            if loc.e != eq
                && loc.terrain_type != eq
                && self.map[loc.y][loc.x][eq].cost > loc.cost + 7
            {
                let mut next_step = loc.clone();
                next_step.cost += 7;
                next_step.step_back = step_back.clone();
                next_step.e = eq;
                ret.push(next_step);
            }
        }

        if loc.x > 0 {
            let mut next_step = self.map[loc.y][loc.x - 1][loc.e as usize].clone();
            if next_step.e != next_step.terrain_type && loc.cost + 1 < next_step.cost {
                next_step.cost = loc.cost + 1;
                next_step.step_back = step_back.clone();
                ret.push(next_step);
            }
        }

        if loc.y > 0 && self.map[loc.y - 1][loc.x][loc.e as usize].cost >= loc.cost + 1 {
            let mut next_step = self.map[loc.y - 1][loc.x][loc.e as usize].clone();
            if next_step.e != next_step.terrain_type && loc.cost + 1 < next_step.cost {
                next_step.cost = loc.cost + 1;
                next_step.step_back = step_back.clone();
                ret.push(next_step);
            }
        }

        if self.map[loc.y][loc.x + 1][loc.e as usize].cost >= loc.cost + 1 {
            let mut next_step = self.map[loc.y][loc.x + 1][loc.e as usize].clone();
            if next_step.e != next_step.terrain_type && loc.cost + 1 < next_step.cost {
                next_step.cost = loc.cost + 1;
                next_step.step_back = step_back.clone();
                ret.push(next_step);
            }
        }

        if self.map[loc.y + 1][loc.x][loc.e as usize].cost >= loc.cost + 1 {
            let mut next_step = self.map[loc.y + 1][loc.x][loc.e as usize].clone();
            if next_step.e != next_step.terrain_type && loc.cost + 1 < next_step.cost {
                next_step.cost = loc.cost + 1;
                next_step.step_back = step_back.clone();
                ret.push(next_step);
            }
        }

        ret
    }

    fn ensure_size(&mut self, x_max: usize, y_max: usize) {
        if self.map.len() >= y_max + 1 && self.map[y_max].len() >= x_max + 1 {
            return;
        }

        let old_y = self.map.len();
        let new_y = if y_max + 1 < old_y { old_y } else { y_max * 2 };
        let old_x = if old_y == 0 { 0 } else { self.map[0].len() };
        let new_x = if x_max + 1 < old_x { old_x } else { x_max * 2 };

        println!("Expanding map {}x{} -> {}x{}", old_x, old_y, new_x, new_y);

        for y in 0..new_y {
            let x_start = if self.map.len() <= y { 0 } else { old_x };
            if self.map.len() <= y {
                self.map.push(Vec::new());
            }
            for x in x_start..new_x {
                let geo_idx = match (x, y) {
                    (0, 0) => 0,
                    (x, y) if (x, y) == self.target => 0,
                    (0, y) => y * 48271,
                    (x, 0) => x * 16807,
                    (x, y) => {
                        &self.map[y - 1][x][EQP_TORCH as usize].erosion_level
                            * &self.map[y][x - 1][EQP_TORCH as usize].erosion_level
                    }
                };
                let erosion_level = (geo_idx + self.depth) % 20183;
                let target_distance = (abs(self.target.0 as i32 - x as i32)
                    + abs(self.target.1 as i32 - y as i32))
                    as usize;
                let target = self.target;
                self.map[y].push(vec![
                    Loc {
                        x,
                        y,
                        e: 0,
                        erosion_level,
                        terrain_type: erosion_level % 3,
                        cost: std::usize::MAX,
                        step_back: (0, 0, EQP_TORCH),
                        target_distance,
                        target,
                    },
                    Loc {
                        x,
                        y,
                        e: 1,
                        erosion_level,
                        terrain_type: erosion_level % 3,
                        cost: std::usize::MAX,
                        step_back: (0, 0, EQP_TORCH),
                        target_distance,
                        target,
                    },
                    Loc {
                        x,
                        y,
                        e: 2,
                        erosion_level,
                        terrain_type: erosion_level % 3,
                        cost: std::usize::MAX,
                        step_back: (0, 0, EQP_TORCH),
                        target_distance,
                        target,
                    },
                ]);
            }
        }
    }

    fn new(depth: usize, target: (usize, usize), limit: (usize, usize)) -> Self {
        let mut ret = Self {
            map: Vec::new(),
            depth,
            target,
            to_check: BinaryHeap::new(),
            shortest_time: None,
        };

        ret.ensure_size(limit.0, limit.1);
        ret.map[0][0][EQP_TORCH].cost = 0;
        ret.to_check.push(ret.map[0][0][EQP_TORCH]);
        ret
    }

    fn find_shortest_time(&mut self) -> usize {
        let mut check_count: usize = 0;
        while let Some(loc) = self.to_check.pop() {
            check_count += 1;
            // If this path is not better than the current-best path through the same location,
            // discard it.
            if loc.cost > 0 && self.map[loc.y][loc.x][loc.e as usize].cost <= loc.cost {
                continue;
            }

            // If this (partial) path is not better than the current-best complete path to
            // the destination, discard it.
            match self.shortest_time.as_ref() {
                Some(&t) if loc.cost + loc.target_distance >= t => continue,
                _ => (),
            }

            // Log this as the best partial path so far.
            self.map[loc.y][loc.x][loc.e as usize] = loc.clone();

            // If this is the destination, record the best total path time we've seen
            // so far.
            if (loc.x, loc.y, loc.e) == (self.target.0, self.target.1, EQP_TORCH) {
                //println!("Destination, cost: {}, shortest: {:?}", loc.cost, self.shortest_time);
                self.shortest_time = match self.shortest_time {
                    Some(t) if loc.cost >= t => Some(t),
                    _ => Some(loc.cost),
                };
                continue;
            }

            // Add the next path steps to the heap for future examination.
            self.ensure_size(loc.x + 1, loc.y + 1);
            self.to_check.extend(self.next_steps(&loc).into_iter());
        }

        let mut curr = self.map[self.target.1][self.target.0][EQP_TORCH as usize];
        let mut path = Vec::new();
        while curr.step_back != (curr.x, curr.y, curr.e) {
            path.push(curr);
            curr = self.map[curr.step_back.1][curr.step_back.0][curr.step_back.2 as usize];
        }
        /*
                for loc in path.iter().rev() {
                    let eq_char = match loc.e {
                        EQP_T => 'T',
                        EQP_C => 'C',
                        EQP_E => 'E',
                        _ => '?',
                    };
                    println!("({}, {}) {}", loc.x, loc.y, eq_char);
                }
        */

        println!("Complete. Loops={}", check_count);
        if let Some(ref t) = self.shortest_time {
            assert_eq!(
                self.map[self.target.1][self.target.0][EQP_TORCH as usize].cost,
                *t
            );
            return *t;
        } else {
            unreachable!()
        }
    }
}

#[aoc(day22, part2, reimpl)]
fn solve_part2_reimpl(_input: &str) -> usize {
    let mut map = Map::new(
        DEPTH,
        TARGET,
        (max(TARGET.0, TARGET.1), max(TARGET.0, TARGET.1)),
    );
    map.find_shortest_time()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_path_cost() {
        let mut map = Map::new(510, (10, 10), (10, 10));
        assert_eq!(45, map.find_shortest_time());
    }
}
