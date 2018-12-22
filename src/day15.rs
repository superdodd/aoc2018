use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::collections::HashMap;

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

    fn find_adjacent_target(&self, i: usize) -> Option<(usize, usize)> {
        let e = &self.entities[i];
        for (x, y) in [
            (e.x, e.y - 1),
            (e.x - 1, e.y),
            (e.x + 1, e.y),
            (e.x, e.y + 1),
        ]
        .iter()
        {
            let d = self.map[*y][*x];
            if d == 'G' || d == 'E' && d != e.entity_type {
                return Some((*x, *y));
            }
        }
        None
    }

    fn move_toward_enemy(&mut self, i: usize) {
        let map = self.get_map_with_entities();
        println!("{}", self);
        let mut to_check: Vec<(usize, usize)> = vec![(self.entities[i].y, self.entities[i].x)];
        let target_type = if self.entities[i].entity_type == 'G' {
            'E'
        } else {
            'G'
        };
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut found_paths: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        let mut shortest_path: Option<Vec<(usize, usize)>> = None;
        found_paths.insert((self.entities[i].y, self.entities[i].x), Vec::new());
        while !to_check.is_empty() {
            let candidate = to_check.remove(0);
            println!("Checking: {:?} ({:?})", candidate, to_check);
            let path = found_paths.get(&candidate).expect("No path").to_vec();
            if map[candidate.0][candidate.1] == target_type {
                // We found a path to an enemy.  Check to see if it's "better" than any we have
                // found already
                println!("Candidate path: {:?}", path);
                shortest_path = match shortest_path {
                    None => {
                        println!("NEW");
                        Some(path.to_vec())
                    },
                    Some(s) => {
                        // If this path is shorter, use it
                        match path.len().cmp(&s.len()) {
                            Ordering::Less => {
                                println!("SHORTER");
                                Some(path.to_vec())
                            }
                            Ordering::Greater => Some(s),
                            Ordering::Equal => {
                                // If paths are the same length, earlier-in-order target is preferable
                                match path.last().unwrap().cmp(s.last().unwrap()) {
                                    Ordering::Less => {
                                        println!("BETTER TARGET");
                                        Some(path.to_vec())
                                    }
                                    Ordering::Greater => Some(s),
                                    // If paths are the same length to the same target, earlier-in-order first
                                    // step is preferable
                                    Ordering::Equal => match path.first().unwrap().cmp(s.first().unwrap()) {
                                        Ordering::Less => {
                                            println!("BETTER STEP");
                                            Some(path.to_vec())
                                        }
                                        Ordering::Greater | Ordering::Equal => Some(s),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for (x, y) in &[
                (candidate.1, candidate.0 - 1),
                (candidate.1 - 1, candidate.0),
                (candidate.1 + 1, candidate.0),
                (candidate.1, candidate.0 + 1),
            ] {
                if visited.contains(&(*y, *x)) {
                    continue;
                }
                if map[*y][*x] == target_type || map[*y][*x] == '.' {
                    let mut new_path = path.to_vec();
                    new_path.push((*y, *x));
                    found_paths.insert((*y, *x), new_path);
                    if !to_check.contains(&(*y, *x)) {
                        to_check.push((*y, *x));
                    }
                }
            }
            visited.push(candidate);
            println!("Done visiting {:?} Left: {:?}", candidate, to_check);
        }
        match shortest_path {
            None => println!("Done with search; no enemy found. Visited: {:?}", visited),
            Some(p) => {
                self.entities[i].x = p.first().unwrap().1;
                self.entities[i].y = p.first().unwrap().0;
            }
        }
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
    atk: i32,
}

impl Entity {
    fn new(x: usize, y: usize, entity_type: char) -> Entity {
        Entity {
            x,
            y,
            entity_type,
            hp: 200,
            atk: 3,
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}:({:2},{:2}) {:3}/{}",
            self.entity_type, self.x, self.y, self.hp, self.atk
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
        (self.y, self.x, self.entity_type, self.hp, self.atk).cmp(&(
            other.y,
            other.x,
            other.entity_type,
            other.hp,
            other.atk,
        ))
    }
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> MapState {
    MapState::parse(input)
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
                atk: 3
            }
        );
    }

    #[test]
    fn test_move_toward_enemy() {
        let mut map = MapState::parse("#######\n#..E..#\n#.#E#.#\n#..G..#\n#######\n");
        map.move_toward_enemy(0);
        assert_eq!(
            map.entities[0],
            Entity {
                x: 6,
                y: 1,
                entity_type: 'E',
                hp: 200,
                atk: 3,
            });
    }
}
