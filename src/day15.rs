use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;

fn get_adjacent(x: usize, y: usize) -> Vec<(usize, usize)> {
    Vec::from(vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)])
}

fn get_adjacent_open(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    get_adjacent(x, y)
        .iter()
        .filter(|(x, y)| map[*y][*x] == '.')
        .map(|x| x.to_owned())
        .collect()
}

fn path_compare_keys(p: &Vec<(usize, usize)>) -> (usize, (usize, usize), (usize, usize)) {
    (
        p.len(),
        {
            let f = p
                .first()
                .or(Some(&(std::usize::MAX, std::usize::MAX)))
                .unwrap();
            (f.1, f.0)
        },
        {
            let f = p
                .last()
                .or(Some(&(std::usize::MAX, std::usize::MAX)))
                .unwrap();
            (f.1, f.0)
        },
    )
}

fn compare_paths(a: &Vec<(usize, usize)>, b: &Vec<(usize, usize)>) -> Ordering {
    match path_compare_keys(a).cmp(&path_compare_keys(b)) {
        Ordering::Equal => {
            // Paths are same length and have same start and end points.  Find the first nonequal
            // elements along the two paths return the comparison between those elements.
            match a.iter().zip(b.iter()).find(|(&a, &b)| a != b) {
                Some((a, b)) => (a.1, a.0).cmp(&(b.1, b.0)),
                None => Ordering::Equal,
            }
        }
        o => o,
    }
}

#[derive(Clone, Debug)]
pub struct MapState {
    map: Vec<Vec<char>>,
    entities: Vec<Entity>,
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let out = self.get_map_with_entities();
        let mut entities = self.entities.to_owned();
        entities.sort();

        for (y, line) in out.iter().enumerate() {
            write!(f, "{}   ", line.iter().collect::<String>())?;
            for e in &entities {
                if e.y == y {
                    write!(f, "{}({}), ", e.entity_type, e.hp)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MapState {
    fn get_map_with_entities(&self) -> Vec<Vec<char>> {
        let mut out = self.map.to_owned();
        for e in &self.entities {
            out[e.y][e.x] = e.entity_type;
        }
        out
    }

    fn entity_index_at(&self, x: usize, y: usize) -> Option<usize> {
        self.entities
            .iter()
            .enumerate()
            .find(|(_i, e)| e.x == x && e.y == y)
            .map(|(i, _e)| i)
    }

    fn find_adjacent_target(&self, i: usize) -> Option<usize> {
        let e = self.entities[i];
        get_adjacent(e.x, e.y).iter()
            .filter_map(|(x, y)| self.entity_index_at(*x, *y).and_then(|i| Some((i, self.entities[i]))))
            .filter(|(_j, o)| e.entity_type != o.entity_type)
            .min_by(|a, b| a.1.hp.cmp(&b.1.hp))
            .and_then(|(j, _o)| Some(j))
    }

    // Find the shortest unobstructed path from the source to the destination.
    fn find_shortest_path(
        &self,
        src: &(usize, usize),
        dst: &(usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let map = self.get_map_with_entities();
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut to_check = Vec::from(get_adjacent_open(src.0, src.1, &map));
        let mut partial_paths: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        while !to_check.is_empty() {
            let candidate = to_check.remove(0);
            visited.push(candidate);
            let path_to_candidate = partial_paths
                .entry(candidate)
                .or_insert(Vec::from(vec![candidate]))
                .clone();

            if candidate == *dst {
                return Some(path_to_candidate.to_owned());
            }

            for next_step in get_adjacent_open(candidate.0, candidate.1, &map)
                .iter().filter(|p| !visited.contains(*p)) {
                partial_paths
                    .entry(*next_step)
                    .or_insert(path_to_candidate.to_owned())
                    .push(*next_step);
                to_check.push(*next_step);
            }
        }
        // Didn't find a path to the destination
        None
    }

    fn move_toward_enemy(&mut self, i: usize) -> bool {
        let map = self.get_map_with_entities();
        let me = self.entities[i];

        // Find all target entities, and the (unique) open squares adjacent to them
        // Then calculate the best path to each open square
        // And finally get the best path among the best paths
        let best_path = self
            .entities
            .iter()
            .enumerate()
            .filter(|(j, o)| i != *j && o.entity_type != me.entity_type)
            .flat_map(|(_j, o)| get_adjacent_open(o.x, o.y, &map))
            .collect::<HashSet<(usize, usize)>>()
            .iter()
            .filter_map(|d| self.find_shortest_path(&(me.x, me.y), d))
            .min_by(compare_paths);

        // If such a path exists, move along it.
        match best_path {
            None => false,
            Some(p) => {
                let step = *p.first().unwrap();
                //println!("{:?} -> {:?}", self.entities[i], step);
                {
                    let e = &mut self.entities[i];
                    e.take_step(step);
                }
                true
            }
        }
    }

    // Execute a turn for the given entity index.
    fn entity_turn(&mut self, i: usize) {
        // First search for adjacent targets to attack.
        // If no target found, try to move and then find another target.
        // If we have a target, attack it.
        match self.find_adjacent_target(i) {
            None => {
                self.move_toward_enemy(i);
                self.find_adjacent_target(i)
            }
            t => t,
        }.map(|t| {
            //println!("{:?} attacking {:?}", self.entities[i], self.entities[t]);
            {
                let e = &mut self.entities[t];
                e.hp -= 3;
            }
        });
    }

    // Execute a full round of turns.  Returns false if the round ends early.
    fn execute_round(&mut self) -> bool {
        let mut i: usize = 0;
        self.entities.sort_by(|a, b| ((a.y, a.x)).cmp(&(b.y, b.x)));
        while i < self.entities.len() {
            // Entity i takes its turn
            self.entity_turn(i);
            // Find any dead entities and remove them from the list
            // before taking the next turn.
            let j = self.entities
                .iter()
                .enumerate()
                .find(|(_j, e)| e.hp <= 0).map(|(j, _e)| j);
            j.map(|j| {
                self.entities.remove(j);
                if j < i {
                    i -= 1;
                }
            });
            i += 1;
            // If there are no enemies left but not everyone has had their turn, end the round early.
            if i < self.entities.len() &&
                !self.entities.iter().any(|e| e.entity_type != self.entities[i].entity_type) {
                return false;
            }
        }
        true
    }

    fn parse(input: &str) -> MapState {
        let mut ret = MapState {
            map: Vec::new(),
            entities: Vec::new(),
        };
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut map_row = Vec::new();
        for c in input.chars() {
            match c {
                'G' | 'E' => {
                    ret.entities.push(Entity::new(x, y, c));
                    map_row.push('.');
                    x += 1;
                }
                '.' | '#' => {
                    map_row.push(c);
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1;
                    ret.map.push(map_row);
                    map_row = Vec::new();
                }
                _ => panic!("Unexpected map input: {}", c),
            }
        }
        if !map_row.is_empty() {
            ret.map.push(map_row);
        }
        ret
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Entity {
    entity_type: char,
    x: usize,
    y: usize,
    hp: i32,
}

impl Entity {
    fn new(x: usize, y: usize, entity_type: char) -> Entity {
        Entity {
            x,
            y,
            entity_type,
            hp: 200,
        }
    }

    fn take_step(&mut self, p: (usize, usize)) {
        assert_eq!(
            1,
            (max(p.0, self.x) - min(p.0, self.x)) + (max(p.1, self.y) - min(p.1, self.y))
        );
        self.x = p.0;
        self.y = p.1;
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}:({:2},{:2}) {:3}",
            self.entity_type, self.x, self.y, self.hp
        )?;
        Ok(())
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x, self.entity_type, self.hp).cmp(&(
            other.y,
            other.x,
            other.entity_type,
            other.hp,
        ))
    }
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> MapState {
    MapState::parse(input)
}

#[aoc(day15, part1)]
pub fn solve_part1(map: &MapState) -> i32 {
    let mut map: MapState = map.to_owned();
    let mut round = 0;
    while map.execute_round() {
        round += 1;
        //println!("Round {}\n{}", round, map);
        if round > 1000 {
            break
        }
    }
    let score = round * map.entities.iter().fold(0, |acc, e| acc + e.hp);
    println!("Round {}, score: {} map:\n{}", round, score, map);
    score
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT_1: &str = "#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######\n";
    const TEST_OUTPUT_1: &str = "#######\n#...#E#\n#E#...#\n#.E##.#\n#E..#E#\n#.....#\n#######\n";

    #[test]
    fn test_map_parse() {
        let map = MapState::parse(TEST_INPUT_1);
        println!("{}", map);
        assert_eq!(
            map.entities[0],
            Entity {
                x: 1,
                y: 1,
                entity_type: 'G',
                hp: 200,
            }
        );
    }

    #[test]
    fn test_compare_paths() {
        //  Paths, in descending order
        let test_paths: Vec<Vec<(usize, usize)>> = vec![
            vec![(0, 0), (1, 0), (1, 1), (2, 1), (1, 1)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 1)],
            vec![(0, 0), (0, 1), (1, 1)],
            vec![(0, 0), (1, 0), (1, 1)],
        ];

        for i in 0..test_paths.len() - 1 {
            for j in i + 1..test_paths.len() {
                assert_eq!(
                    Ordering::Greater,
                    compare_paths(&test_paths[i], &test_paths[j])
                );
                assert_eq!(
                    Ordering::Less,
                    compare_paths(&test_paths[j], &test_paths[i])
                );
                assert_eq!(
                    Ordering::Equal,
                    compare_paths(&test_paths[i], &test_paths[i])
                );
                assert_eq!(
                    Ordering::Equal,
                    compare_paths(&test_paths[j], &test_paths[j])
                );
            }
        }
    }

    #[test]
    fn test_move_toward_enemy() {
        let mut map = MapState::parse("#######\n#..E..#\n#.#E#.#\n#..G..#\n#######\n");
        map.move_toward_enemy(0);
        assert_eq!(
            map.entities[0],
            Entity {
                x: 2,
                y: 1,
                entity_type: 'E',
                hp: 200,
            }
        );
    }

    #[test]
    fn test_part1_solution() {
        let Inputs = [
            ("#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######\n", 27730),
            ("#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######\n", 36334)
        ];

        for (i, t) in Inputs.iter().enumerate() {
            let map = MapState::parse(t.0);
            println!("{}\n{}", i, map);
            assert_eq!(t.1, solve_part1(&map));
        }
    }

    #[test]
    fn test_in_place_modify() {
        let mut _map = MapState::parse(TEST_INPUT_1);
    }
}
