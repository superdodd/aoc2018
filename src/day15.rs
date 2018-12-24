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
    vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
}

fn get_adjacent_open(x: usize, y: usize, map: &[Vec<char>]) -> Vec<(usize, usize)> {
    get_adjacent(x, y)
        .iter()
        .filter(|(x, y)| map[*y][*x] == '.')
        .map(|x| x.to_owned())
        .collect()
}

fn path_compare_keys(p: &[(usize, usize)]) -> (usize, (usize, usize), (usize, usize)) {
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

fn compare_paths(a: &[(usize, usize)], b: &[(usize, usize)]) -> Ordering {
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

#[derive(PartialEq, Debug)]
enum EndState {
    NotFinished,
    NoEnemies,
    ElfDied,
}

#[derive(Clone, Debug)]
pub struct MapState {
    map: Vec<Vec<char>>,
    entities: Vec<Entity>,
    all_elves_live: bool,
    elf_atk: i32,
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

    // Find the best path from the source to one or more optional destinations.
    fn find_shortest_path(
        &self,
        src: &(usize, usize),
        dst: &[(usize, usize)],
    ) -> Option<Vec<(usize, usize)>> {
        //println!("Find path: {:?} -> {:?}", src, dst);
        // Need this handy to find open adjacent squares
        let map = self.get_map_with_entities();
        // The set of locations already visited by the algorithm
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        // The remaining destinations to find paths for
        let mut destinations_to_find: HashSet<&(usize, usize)> = HashSet::new();
        for d in dst {
            destinations_to_find.insert(d);
        }
        // The best found path so far among all destinations
        let mut best_found: Option<Vec<(usize, usize)>> = None;
        // The current queue of locations to build paths for
        let mut to_check = get_adjacent_open(src.0, src.1, &map);
        // The set of partial paths built so far
        let mut partial_paths: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        //println!("before: to-check={:?} destinations-to-find={:?}", to_check, destinations_to_find);
        while !to_check.is_empty() && !destinations_to_find.is_empty() {
            let candidate = to_check.remove(0);
            visited.insert(candidate.to_owned());
            let path_to_candidate = partial_paths
                .entry(candidate)
                .or_insert_with(|| vec![candidate])
                .clone();

            if destinations_to_find.contains(&candidate) {
                //println!("{:?} -> {:?} = {:?}", src, candidate, path_to_candidate);
                best_found = best_found.map_or(Some(path_to_candidate.to_owned()), |b| match compare_paths(&path_to_candidate.to_owned(), &b) {
                    Ordering::Less => Some(path_to_candidate.to_owned()),
                    Ordering::Equal => panic!("Duplicate paths found"),
                    _ => Some(b),
                });
                //println!("Best = {:?}", best_found);
                destinations_to_find.remove(&candidate);
            }

            for next_step in get_adjacent_open(candidate.0, candidate.1, &map) {
                if !visited.contains(&next_step) && !to_check.contains(&next_step) {
                    partial_paths
                        .entry(next_step)
                        .or_insert_with(|| path_to_candidate.to_owned())
                        .push(next_step);
                    to_check.push(next_step);
                }
            }
        }
        //println!("after: to-check={:?} destinations-to-find={:?}", to_check, destinations_to_find);
        best_found
    }

    fn move_toward_enemy(&mut self, i: usize) -> bool {
        let map = self.get_map_with_entities();
        let me = self.entities[i];

        // Find all target entities, and the (unique) open squares adjacent to them
        let mut targets_unique =
            self.entities.iter().filter(|e| e.entity_type != me.entity_type)
                .flat_map(|e| get_adjacent_open(e.x, e.y, &map))
                .collect::<Vec<(usize, usize)>>();
        targets_unique.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
        targets_unique.dedup();
        // Then calculate the best path to any open square.
        // If such a path exists, move along it.
        match self.find_shortest_path(&(me.x, me.y), targets_unique.as_slice()) {
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
        let t = match self.find_adjacent_target(i) {
            None => {
                self.move_toward_enemy(i);
                self.find_adjacent_target(i)
            }
            t => t,
        };
        if let Some(t) = t {
            let e = &mut self.entities[t];
            if e.entity_type == 'G' {
                e.hp -= self.elf_atk;
            } else {
                e.hp -= 3;
            }
        }
    }

    // Execute a full round of turns.  Returns false if the round ends early.
    fn execute_round(&mut self) -> EndState {
        let mut i: usize = 0;
        self.entities.sort_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
        // If we start the round with only one entity, we don't need to do anything.
        if self.entities.len() == 1 {
            return EndState::NoEnemies;
        }
        while i < self.entities.len() {
            // Entity i takes its turn
            self.entity_turn(i);
            // Find any dead entities and remove them from the list
            // before taking the next turn.
            let j = self.entities
                .iter()
                .enumerate()
                .find(|(_j, e)| e.hp <= 0).map(|(j, _e)| j);
            if let Some(j) = j {
                if self.all_elves_live && self.entities[j].entity_type == 'E' {
                    return EndState::ElfDied;
                }
                self.entities.remove(j);
                if j < i {
                    i -= 1;
                }
            };
            i += 1;
            // If there are no enemies left but not everyone has had their turn, end the round early.
            if i < self.entities.len() &&
                !self.entities.iter().any(|e| e.entity_type != self.entities[i].entity_type) {
                return EndState::NoEnemies;
            }
        }
        EndState::NotFinished
    }

    fn parse(input: &str) -> MapState {
        let mut ret = MapState {
            map: Vec::new(),
            entities: Vec::new(),
            all_elves_live: false,
            elf_atk: 3,
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

    fn set_rules(&mut self, all_elves_live: bool, elf_atk: i32) {
        self.all_elves_live = all_elves_live;
        self.elf_atk = elf_atk;
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
    let map = MapState::parse(input);
    println!("{}", map);
    map
}

#[aoc(day15, part1)]
pub fn solve_part1(map: &MapState) -> i32 {
    let mut map: MapState = map.to_owned();
    let mut round = 0;
    while map.execute_round() == EndState::NotFinished {
        round += 1;
        let score = round * map.entities.iter().fold(0, |acc, e| acc + e.hp);
        println!("Round {}, score {}\n{}", round, score, map);
    }
    let score = round * map.entities.iter().fold(0, |acc, e| acc + e.hp);
    println!("DONE. Round {}, score: {}:\n{}", round, score, map);
    score
}

#[aoc(day15, part2)]
pub fn solve_part2(in_map: &MapState) -> i32 {
    let mut atk_power = 3;
    let mut round= 0;
    let mut state = EndState::ElfDied;
    let mut map: MapState = in_map.to_owned();
    while state == EndState::ElfDied {
        map = in_map.to_owned();
        atk_power += 1;
        map.set_rules(true, atk_power);
        round = 0;
        while {
            state = map.execute_round();
            state == EndState::NotFinished
        } {
            round += 1;
        }
        println!("Atk={:?}, R={:?} -> {:?}", atk_power, round, state);
    }
    round * map.entities.iter().fold(0, |acc, e| acc + e.hp)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT_1: &str = "#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######\n";
    //const TEST_OUTPUT_1: &str = "#######\n#...#E#\n#E#...#\n#.E##.#\n#E..#E#\n#.....#\n#######\n";

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
        let inputs = [
            ("#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######\n", 27730),
            ("#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######\n", 36334),
            ("#######\n#E..EG#\n#.#G.E#\n#E.##E#\n#G..#.#\n#..E#.#\n#######\n", 39514),
            ("#######\n#E.G#.#\n#.#G..#\n#G.#.G#\n#G..#.#\n#...E.#\n#######\n", 27755),
            ("#######\n#.E...#\n#.#..G#\n#.###.#\n#E#G#G#\n#...#G#\n#######\n", 28944),
            ("#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########\n", 18740),
        ];

        for (i, t) in inputs.iter().enumerate() {
            let map = MapState::parse(t.0);
            println!("{}\n{}", i, map);
            assert_eq!(t.1, solve_part1(&map));
        }
    }

    #[test]
    fn test_part2_solution() {
        let inputs = [
            ("#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########\n", 1140),
        ];

        for (_i, t) in inputs.iter().enumerate() {
            let map = MapState::parse(t.0);
            assert_eq!(t.1, solve_part2(&map));
        }
    }

    #[test]
    fn test_in_place_modify() {
        let mut _map = MapState::parse(TEST_INPUT_1);
    }

    #[test]
    fn test_move() {
        let mut map = MapState::parse("########\n#....G.#\n#..G...#\n##.....#\n###.E..#\n####...#\n########\n");
        println!("{}", map);
        map.execute_round();
        println!("{}", map);
        assert_eq!((4,2), (map.entities[1].x, map.entities[1].y));

    }
}
