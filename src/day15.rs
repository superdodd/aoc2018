use aoc_runner_derive::{aoc,aoc_generator};
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct MapState {
    map: Vec<Vec<char>>,
    entities: Vec<Entity>,
}

impl fmt::Display for MapState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut out = self.get_map_with_entities();
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



    fn parse(input: &str) -> MapState {
        let mut ret = MapState{
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
                },
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
        Entity{x, y, entity_type, hp: 200, atk: 3}
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}:({:2},{:2}) {:3}/{}", self.entity_type, self.x, self.y, self.hp, self.atk)?;
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
        (self.y, self.x, self.entity_type, self.hp, self.atk)
            .cmp(&(other.y, other.x, other.entity_type, other.hp, other.atk))
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
        assert_eq!(map.entities[0], Entity{x: 1, y: 1, entity_type: 'G', hp: 200, atk: 3});
    }
}