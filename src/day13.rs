use aoc_runner_derive::{aoc, aoc_generator};
use std::mem::replace;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cart {
    x: i32,
    y: i32,
    direction: Direction,
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({}, {}) {:?}]", self.x, self.y, self.direction)
    }
}

pub struct Map {
    carts: Vec<Cart>,
    map: Vec<Vec<char>>,
}

impl Map {
    fn from_u8array(input: &[u8]) -> Map {
        let mut ret = Map {
            carts: Vec::new(),
            map: Vec::new(),
        };
        let mut x = 0;
        let mut y = 0;
        let mut row: Vec<char> = Vec::new();
        for c in input {
            match char::from(*c) {
                '^' => {
                    ret.carts.push(Cart {
                        x: x,
                        y: y,
                        direction: Direction::UP,
                    });
                    row.push('|');
                }
                'v' => {
                    ret.carts.push(Cart {
                        x: x,
                        y: y,
                        direction: Direction::DOWN,
                    });
                    row.push('|');
                }
                '>' => {
                    ret.carts.push(Cart {
                        x: x,
                        y: y,
                        direction: Direction::RIGHT,
                    });
                    row.push('-');
                }
                '<' => {
                    ret.carts.push(Cart {
                        x: x,
                        y: y,
                        direction: Direction::LEFT,
                    });
                    row.push('-');
                }
                '\n' => {
                    let r: Vec<char> = replace(&mut row, Vec::new());
                    ret.map.push(r);
                    y += 1;
                    x = -1;
                }
                _ => row.push(c.clone() as char),
            }
            x += 1;
        }
        ret
    }
}

#[aoc_generator(day13)]
pub fn parse(input: &[u8]) -> Map {
    Map::from_u8array(input)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Map) -> Cart {
    input.carts.get(0).expect("Cart not found").clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = b"/->-\\\n|   |  /----\\\n    | /-+--+-\\  |\n    | | |  | v  |\n    \\-+-/  \\-+--/\n      \\------/";

    #[test]
    fn part1_example() {
        let map = parse(INPUT);

        let c: &Cart = map.carts.get(0).expect("No cart found");
        assert_eq!(c, &Cart{x:0, y:3, direction: Direction::RIGHT}, "Incorrect first cart");
    }
}
