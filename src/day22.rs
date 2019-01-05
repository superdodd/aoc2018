use aoc_runner_derive::aoc;
use std::mem;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp::min;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Write;

const DEPTH: usize = 11820;
const TARGET: (usize, usize) = (7, 782);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct MapCell(usize);

const REG_R: MapCell = MapCell(0); // Rocky
const REG_W: MapCell = MapCell(1); // Wet
const REG_N: MapCell = MapCell(2); // Narrow

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char(match self {
            &REG_R => '.',
            &REG_W => '=',
            &REG_N => '|',
            _ => '?',
        })
    }
}

impl fmt::Debug for MapCell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Eqp(usize);

const EQP_E: Eqp = Eqp(0); // Empty hands
const EQP_T: Eqp = Eqp(1); // Torch equipped
const EQP_C: Eqp = Eqp(2); // Climbing gear equipped

impl fmt::Display for Eqp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char(match self {
            &EQP_E => 'E',
            &EQP_T => 'T',
            &EQP_C => 'C',
            _ => '?',
        })
    }
}

impl fmt::Debug for Eqp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

struct Map {
    map: Vec<Vec<MapCell>>,
    depth: usize,
    target: (usize, usize),
}

impl Map {
    fn ensure_size(&mut self, x: usize, y: usize) {
        // Expand our map if we need to. Assume the row vectors all have the same size.
        if y >= self.map.len() || x >= self.map[0].len() {
            mem::replace(map, Self::new(self.depth, self.target, (x, y)));
        }

    }

    fn new(depth: usize, target: (usize, usize), limit: (usize, usize)) -> Self {
        let mut geo_idx = vec![vec![0; limit.0 + 1]; limit.1 + 1];
        for y in 0..=limit.1 {
            for x in 0..=limit.0 {
                let idx: usize = if (x, y) == target {
                    0
                } else {
                    match (x, y) {
                        (0, 0) => 0,
                        (x, 0) => x * 16807,
                        (0, y) => y * 48271,
                        (x, y) => geo_idx[y][x-1] * geo_idx[y-1][x],
                    }
                };
                geo_idx[y][x] = (idx + depth) % 20183;
            }
        }
        let mut ret = vec![vec![MapCell(0); limit.0 + 1]; limit.1 + 1];
        for y in 0..geo_idx.len() {
            for x in 0..geo_idx[y].len() {
                ret[y][x] = MapCell(geo_idx[y][x] % 3);
            }
        }
        Self {
            depth,
            target,
            map: ret,
        }
    }

    fn get_map_str(&self) -> String {
        let mut map_str = String::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                map_str.push(if (x, y) == (0, 0) {
                    'M'
                } else if (x, y) == self.target {
                    'T'
                } else {
                    match self.map[y][x] {
                        REG_R => '.',
                        REG_W => '=',
                        REG_N => '|',
                        _ => '?',
                    }
                });
            }
            map_str.push('\n');
        }
        map_str
    }

    fn get_risk_level(&self) -> usize {
        let mut risk_level: usize = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk_level += self.map[y][x].0;
            }
        }
        risk_level
    }

    fn valid_state(&mut self, x: usize, y: usize, e: Eqp) -> bool {
        self.ensure_size(x, y);
        match self.map[y][x] {
            REG_R if e == EQP_E => false,
            REG_N if e == EQP_C => false,
            REG_W if e == EQP_T => false,
            _ => true,
        }
    }

    fn find_shortest_time(&mut self) -> usize {
        // Do a breadth-first search to find the shortest-time path from the origin to the target.
        // Note that the solution space incorporates both the x, y location as well as the equipped item.
        let mut to_check: Vec<(usize, usize, Eqp)> = Vec::new();
        to_check.push((0, 0, EQP_T));
        // For locations we have already found a path to, store information about the quickest route there.
        let mut paths: HashMap<(usize, usize, Eqp), (usize, Vec<(usize, usize, Eqp)>)> = HashMap::new();
        paths.insert((0, 0, EQP_T), (0, vec![(0, 0, EQP_T)]));
        // The current shortest time to target, if one has been found so far
        let mut shortest_time: Option<usize> = None;
        // Destination is target location and holding the torch
        let destination: (usize, usize, Eqp) = (self.target.0, self.target.1, EQP_T);
        while !to_check.is_empty() {
            let start = to_check.remove(0);
            let curr_path = match paths.entry(start) {
                Entry::Vacant(_) => panic!("Expected path entry"),
                Entry::Occupied(e) => e.get().clone(),
            };
            println!("{:?} ({} more): {:?}", start, to_check.len(), curr_path);

            if start == destination {
                shortest_time.replace(min(shortest_time.unwrap_or(curr_path.0), curr_path.0));
                continue;
            }

            if let Some(t) = shortest_time {
                if t < curr_path.0 {
                    // No need to check anything else. This path is already sub-optimal.
                    continue;
                }
            }

            // Check next path states in order:
            // 1) Switch equipment (cost: 7 minutes)
            // 2) Move if possible: up, left, right, down.
            // Each check_next call potentially adds a new item to the to_check list, updates the entry
            // in paths, and expands the map if required.
            for e in (0..=2).map(|i| Eqp(i)) {
                check_next(&mut map, &mut to_check, &mut paths, &curr_path, &(start.0, start.1, e), &shortest_time);
            }
            if start.1 > 0 {
                check_next(&mut map, &mut to_check, &mut paths, &curr_path, &(start.0, start.1 - 1, start.2), &shortest_time);
            }
            if start.0 > 0 {
                check_next(&mut map, &mut to_check, &mut paths, &curr_path, &(start.0 - 1, start.1, start.2), &shortest_time);
            }
            check_next(&mut map, &mut to_check, &mut paths, &curr_path, &(start.0 + 1, start.1, start.2), &shortest_time);
            check_next(&mut map, &mut to_check, &mut paths, &curr_path, &(start.0, start.1 + 1, start.2), &shortest_time);
        }

        println!("{}", self.get_map_str());
        match paths.entry(destination) {
            Entry::Vacant(_) => panic!("No path to destination"),
            Entry::Occupied(o) => {
                let path = &o.get().1;
                let mut cost = 0;
                let mut prev = (0, 0, EQP_T);
                for i in 0..path.len() {
                    let (x, y, e) = path[i];
                    if prev.2 != e {
                        assert_eq!((x, y), (prev.0, prev.1));
                        cost += 7;
                    } else if prev.0 != x {
                        assert_eq!((y, e), (prev.1, prev.2));
                        cost += 1;
                    } else if prev.1 != y {
                        assert_eq!((x, e), (prev.0, prev.2));
                        cost += 1;
                    }

                    println!("({}, {}) {} {} -> {}", x, y, e, self.map[y][x], cost);
                    prev = path[i];
                }
            }
        }
        shortest_time.unwrap()
    }
}

#[aoc(day22, part1)]
fn solve_part1(_input: &str) -> usize {
    let map = Map::new(DEPTH, TARGET, TARGET);
    map.get_risk_level()
}

/// For the given destination `step`, see if we need to (re-)examine paths through `step`.
/// If so, update `to_check` for future iterations of the main loop. 
fn check_next(map: &mut Map, to_check: &mut Vec<(usize, usize, Eqp)>, paths: &mut HashMap<(usize, usize, Eqp), (usize, Vec<(usize, usize, Eqp)>)>, curr_path: &(usize, Vec<(usize, usize, Eqp)>), step: &(usize, usize, Eqp), shortest_time: &Option<usize>) {
    map.ensure_size(step.0, step.1);
    print!("Checking: {:?} for {:?} ", step, map.map[step.1][step.0]);
    if map.valid_state(step.0, step.1, step.2) {
        println!("... valid");
    } else {
        println!("... invalid");
        return;
    }
    let prev = curr_path.1.last().unwrap();
    if step == prev {
        return;
    }
    let step_cost = if prev.2 == step.2 {
        1
    } else {
        7
    };
    let new_path_cost = curr_path.0 + step_cost;

    if let Some(t) = *shortest_time {
        if new_path_cost > t {
            // Already a suboptimal path
            return;
        }
    }

    let mut new_path = curr_path.1.to_owned();
    new_path.push(*step);

    match paths.entry(*step) {
        Entry::Vacant(v) => {
            println!("Adding new: {:?}", step);
            v.insert((new_path_cost, new_path));
            if !to_check.contains(step) {
                to_check.push(*step);
            } else {
                println!("...already in list");
            }
        }
        Entry::Occupied(ref mut o) => {
            let (old_path_cost, old_path) = o.get_mut();
            if new_path_cost < *old_path_cost {
                println!("Found better path {} -> {} {:?}", old_path_cost, new_path_cost, step);
                *old_path_cost = new_path_cost;
                *old_path = new_path;
                if !to_check.contains(step) {
                    to_check.push(*step);
                } else {
                    println!("...already in list");
                }
            }
        }
    }
    println!("{:?}", paths.entry(*step));
}

#[aoc(day22, part2)]
fn solve_part2(_input: &str) -> usize {
    let mut map = get_map(DEPTH, TARGET, TARGET);
    find_shortest_time(&mut map, TARGET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_generate_map() {
        let map = Map::new(510, (10, 10), (15, 15));
        let map_str = map.get_map_str();
        let expected_map = "
M=.|=.|.|=.|=|=.
.|=|=|||..|.=...
.==|....||=..|==
=.|....|.==.|==.
=|..==...=.|==..
=||.=.=||=|=..|=
|.=.===|||..=..|
|..==||=.|==|===
.=..===..=|.|||.
.======|||=|=.|=
.===|=|===T===||
=|||...|==..|=.|
=.=|=.=..=.||==|
||=|=...|==.=|==
|=.=||===.|||===
||.|==.|.|.||=||\n";
        assert_eq!(&expected_map[1..], map_str);
    }

    #[test]
    fn test_part1_calculate_risk() {
        let map = Map::new(510, (10, 10), (15, 15));
        assert_eq!(114, map.get_risk_level());
    }

    #[test]
    fn test_part2_path_cost() {
        let mut map = Map::new(510, (10, 10), (10, 10));
        assert_eq!(45, map.find_shortest_time());
    }

    #[test]
    fn test_valid_states() {
        let mut map = Map::new(510, (10, 10), (10, 10));
        let cases = vec![
            (0, 1, EQP_C, true),
            (0, 1, EQP_T, true),
            (0, 1, EQP_E, false),
            (1, 0, EQP_C, true),
            (1, 0, EQP_T, false),
            (1, 0, EQP_E, true),
            (1, 1, EQP_C, false),
            (1, 1, EQP_E, true),
            (1, 1, EQP_T, true),
        ];
        for t in cases {
            let eqp = match t.2 {
                EQP_E => 'x',
                EQP_C => 'C',
                EQP_T => 'T',
                _ => panic!("Unknown equipment {}", t.2),
            };
            let reg = match map.map[t.1][t.0] {
                REG_R => '.',
                REG_N => '|',
                REG_W => '=',
                _ => panic!("Unknown map cell {}", map.map[t.1][t.0]),
            };

            assert_eq!(t.3, map.valid_state(t.0, t.1, t.2), "({}, {}) {} + {}: expected {}", t.0, t.1, reg, eqp, t.3);
        }
    }
}