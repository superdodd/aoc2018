use aoc_runner_derive::aoc;
//use std::cmp::min;
use std::cmp::max;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;
//use fnv::FnvHashSet;
//use std::collections::HashSet;

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

struct Loc {
    cell: MapCell,
    erosion_level: usize,
    // For each equipment type, see if we need to try to extend a new path from here.
    needs_check: [bool; 3],
    // For each equipment type, store a pointer back
    // to the next step of the traceback to the origin
    // along the shortest (currently known) path.
    paths: [Option<(usize, (usize, usize, Eqp))>; 3],
}

struct Map {
    map: Vec<Vec<Loc>>,
    depth: usize,
    target: (usize, usize),
    to_check: VecDeque<(usize, usize, Eqp)>,
    //to_check_hash: FnvHashSet<(usize, usize, Eqp)>,
    //to_check_hash: HashSet<(usize, usize, Eqp)>,
    shortest_time: Option<usize>,
    destination: (usize, usize, Eqp),
}

impl Map {
    fn new(depth: usize, target: (usize, usize), limit: (usize, usize)) -> Self {
        let mut ret = Self {
            map: Vec::new(),
            depth,
            target,
            to_check: VecDeque::with_capacity(limit.0 * limit.1 * 3),
            //to_check_hash: FnvHashSet::default(),
            //to_check_hash: HashSet::default(),
            shortest_time: None,
            destination: (target.0, target.1, EQP_T),
        };
        //ret.to_check_hash.insert((0, 0, EQP_T));
        ret.ensure_size(max(limit.0, limit.1), max(limit.0, limit.1));
        ret.to_check.push_front((0, 0, EQP_T));
        ret.map[0][0].needs_check[EQP_T.0] = true;
        ret.map[0][0].paths[EQP_T.0] = Some((0, (0, 0, EQP_T)));
        ret
    }

    /// Ensure that we can access coordinate (x_limit, y_limit) on the map.
    /// Expand the map if necessary to accommodate.
    fn ensure_size(&mut self, x_limit: usize, y_limit: usize) {
        let prev_y_size = self.map.len();
        let prev_x_size = if prev_y_size == 0 {
            0
        } else {
            self.map[0].len()
        };
        let new_y_size = y_limit + 1;
        let new_x_size = x_limit + 1;

        if new_y_size <= prev_y_size && new_x_size <= prev_x_size {
            return;
        }
        //println!("size={}x{} expand to {}x{}", prev_x_size, prev_y_size, new_x_size, new_y_size);

        // Attempt to do a single set of allocations if we are expanding the map.
        self.map.reserve(max(new_y_size, prev_y_size) - prev_y_size);
        for y in 0..self.map.len() {
            self.map[y].reserve(max(new_x_size, prev_x_size) - prev_x_size);
        }

        // Fill in any missing values.
        for y in 0..max(prev_y_size, new_y_size) {
            let x_start: usize;
            if y >= self.map.len() {
                //println!("Adding row y={}", y);
                // Need to add a new row... populate it from x=0
                self.map.push(Vec::with_capacity(new_x_size));
                x_start = 0;
            } else {
                // Already a partial row here... populate it from x=prev_x_size
                x_start = prev_x_size;
            }
            // Calculate new elements.  We are adding to len so push onto the row.
            for x in x_start..max(prev_x_size, new_x_size) {
                //println!("Adding: ({}, {})", x, y);
                let idx: usize = if (x, y) == self.target {
                    0
                } else {
                    match (x, y) {
                        (0, 0) => 0,
                        (x, 0) => x * 16807,
                        (0, y) => y * 48271,
                        (x, y) => {
                            self.map[y][x - 1].erosion_level * self.map[y - 1][x].erosion_level
                        }
                    }
                };
                let erosion_level = (idx + self.depth) % 20183;
                self.map[y].push(Loc {
                    needs_check: [false; 3],
                    cell: MapCell(erosion_level % 3),
                    erosion_level,
                    paths: [None; 3],
                });
            }
        }
    }

    #[allow(dead_code)]
    fn get_map_str(&self) -> String {
        let mut map_str = String::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                map_str.push(if (x, y) == (0, 0) {
                    'M'
                } else if (x, y) == self.target {
                    'T'
                } else {
                    match self.map[y][x].cell {
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
                risk_level += self.map[y][x].cell.0;
            }
        }
        risk_level
    }

    fn valid_state(&mut self, x: usize, y: usize, e: Eqp) -> bool {
        match self.map[y][x].cell {
            REG_R if e == EQP_E => false,
            REG_N if e == EQP_C => false,
            REG_W if e == EQP_T => false,
            _ => true,
        }
    }

    fn check_next(
        &mut self,
        start: &(usize, usize, Eqp),
        start_cost: usize,
        step: &(usize, usize, Eqp),
    ) {
        if step == start {
            return;
        }
        let (x, y, e) = *step;
        let step_cost = if start.2 == e { 1usize } else { 7 };
        let new_path_cost = start_cost + step_cost;

        if let Some(t) = self.shortest_time {
            if new_path_cost > t {
                // Already a suboptimal path
                return;
            }
        }

        if !self.valid_state(x, y, e) {
            return;
        }

        let mut loc = &mut self.map[y][x];
        match loc.paths[e.0] {
            Some(existing_path) if existing_path.0 <= new_path_cost => (),
            _ => {
                loc.paths[e.0] = Some((new_path_cost, *start));
                if !loc.needs_check[e.0] {
                    loc.needs_check[e.0] = true;
                    self.to_check.push_back(*step);
                }
            }
        }
    }

    fn find_shortest_time(&mut self) -> usize {
        // Do a breadth-first search to find the shortest-time path from the origin to the target.
        // Note that the solution space incorporates both the x, y location as well as the equipped item.
        while let Some(start) = self.to_check.pop_front() {
            self.map[start.1][start.0].needs_check[(start.2).0] = false;
            let trace_back = &self.map[start.1][start.0].paths[(start.2).0].unwrap();

            if start == self.destination {
                self.shortest_time = match self.shortest_time {
                    Some(t) if t < trace_back.0 => Some(t),
                    _ => Some(trace_back.0),
                };
                continue;
            }

            if let Some(t) = self.shortest_time {
                if t < trace_back.0 {
                    // No need to check anything else. This path is already sub-optimal.
                    continue;
                }
            }

            // Check next path states in order:
            // 1) Switch equipment (cost: 7 minutes)
            // 2) Move if possible: up, left, right, down.
            // Each check_next call potentially adds a new item to the to_check list, updates the entry
            // in paths, and expands the map if required.
            self.ensure_size(start.0 + 1, start.1 + 1);
            self.check_next(&start, trace_back.0, &(start.0, start.1, EQP_T));
            self.check_next(&start, trace_back.0, &(start.0, start.1, EQP_C));
            self.check_next(&start, trace_back.0, &(start.0, start.1, EQP_E));
            if start.1 > 0 {
                self.check_next(&start, trace_back.0, &(start.0, start.1 - 1, start.2));
            }
            if start.0 > 0 {
                self.check_next(&start, trace_back.0, &(start.0 - 1, start.1, start.2));
            }
            self.check_next(&start, trace_back.0, &(start.0 + 1, start.1, start.2));
            self.check_next(&start, trace_back.0, &(start.0, start.1 + 1, start.2));
        }

        //println!("{}", self.get_map_str());
        if let Some(best_path) =
            &self.map[self.destination.1][self.destination.0].paths[(self.destination.2).0]
        {
            let mut cost = 0;
            let mut prev = self.destination;
            let mut curr = best_path.1;
            while prev != (0, 0, EQP_T) {
                let (x, y, e) = curr;
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
                prev = curr;
                curr = self.map[curr.1][curr.0].paths[(curr.2).0].unwrap().1;
            }
            assert_eq!(cost, best_path.0);
        } else {
            panic!("No best path found");
        }
        self.shortest_time.unwrap()
    }
}

#[aoc(day22, part1)]
fn solve_part1(_input: &str) -> usize {
    let map = Map::new(DEPTH, TARGET, TARGET);
    map.get_risk_level()
}

#[aoc(day22, part2, orig)]
fn solve_part2(_input: &str) -> usize {
    let mut map = Map::new(DEPTH, TARGET, TARGET);
    map.find_shortest_time()
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
            let reg = match map.map[t.1][t.0].cell {
                REG_R => '.',
                REG_N => '|',
                REG_W => '=',
                _ => panic!("Unknown map cell {}", map.map[t.1][t.0].cell),
            };

            assert_eq!(
                t.3,
                map.valid_state(t.0, t.1, t.2),
                "({}, {}) {} + {}: expected {}",
                t.0,
                t.1,
                reg,
                eqp,
                t.3
            );
        }
    }
}
