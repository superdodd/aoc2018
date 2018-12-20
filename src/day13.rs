use aoc_runner_derive::{aoc, aoc_generator};
use num_traits::FromPrimitive;
use std::cmp::min;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::mem::replace;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Copy, Clone, FromPrimitive, PartialEq, Eq, Ord, PartialOrd)]
enum TurnState {
    LEFT,
    STRAIGHT,
    RIGHT,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Cart {
    x: i32,
    y: i32,
    direction: Direction,
    turnState: TurnState,
}

impl Cart {
    fn new(x: i32, y: i32, c: u8) -> Cart {
        Cart {
            x,
            y,
            direction: match c as char {
                '^' => Direction::UP,
                'v' => Direction::DOWN,
                '>' => Direction::RIGHT,
                '<' => Direction::LEFT,
                _ => panic!("Invalid cart character: {}", c),
            },
            turnState: TurnState::LEFT,
        }
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({}, {}) {:?}]", self.x, self.y, self.direction)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            return None;
        }
        if self.y == other.y {
            return Some(self.x.cmp(&other.x));
        }
        Some(self.y.cmp(&other.y))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(&other) {
            r @ Some(_) => r.unwrap(),
            None => {
                if self.direction == other.direction {
                    return self.turnState.cmp(&other.turnState);
                }
                self.direction.cmp(&other.direction)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    carts: Vec<Cart>,
    map: Vec<Vec<char>>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Carts:")?;
        for i in 0..self.carts.len() {
            let c = &self.carts[i];
            writeln!(f, "  {:?}", c)?;
        }
        writeln!(f, "Map:\n{}", self.get_map())?;
        Ok(())
    }
}

fn check_collision(carts: &Vec<Cart>, i: usize) -> Option<usize> {
    let x = carts[i].x;
    let y = carts[i].y;
    for (j, cart) in carts.iter().enumerate() {
        if i == j {
            continue;
        }
        if cart.x == x && cart.y == y {
            return Some(j);
        }
    }
    None
}

impl Map {
    fn get_map(&self) -> String {
        let mut outmap = self.map.clone();
        for i in 0..self.carts.len() {
            let c = &self.carts[i];
            outmap[c.y as usize][c.x as usize] = match c.direction {
                Direction::LEFT => '<',
                Direction::RIGHT => '>',
                Direction::UP => '^',
                Direction::DOWN => 'v',
            }
        }
        outmap
            .iter()
            .map(|l| l.iter().collect::<String>() + "\n")
            .collect()
    }

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
                '^' | 'v' => {
                    ret.carts.push(Cart::new(x, y, *c));
                    row.push('|');
                }
                '>' | '<' => {
                    ret.carts.push(Cart::new(x, y, *c));
                    row.push('-');
                }
                '\n' => {
                    let r: Vec<char> = replace(&mut row, Vec::new());
                    ret.map.push(r);
                    y += 1;
                    x = -1;
                }
                ' ' | '-' | '|' | '/' | '\\' | '+' => row.push(*c as char),
                _ => panic!("Unexpected map character {}", *c),
            }
            x += 1;
        }
        if !row.is_empty() {
            ret.map.push(row);
        }
        ret
    }

    fn update(&mut self, clear_collisions: bool) -> Option<Cart> {
        // First, ensure the carts are sorted.
        self.carts.sort();

        let mut i: usize = 0;
        while i < self.carts.len() {
            let mut x: i32;
            let mut y: i32;
            // Update each cart state using a mutable reference to the current cart.
            {
                let mut cart: Cart = self.carts[i];
                // Move the cart along the track..
                match cart.direction {
                    Direction::UP => cart.y -= 1,
                    Direction::DOWN => cart.y += 1,
                    Direction::LEFT => cart.x -= 1,
                    Direction::RIGHT => cart.x += 1,
                }
                // Turn the cart if needed.
                cart.direction = match (self.map[cart.y as usize][cart.x as usize], cart.direction)
                {
                    ('/', Direction::UP) => Direction::RIGHT,
                    ('/', Direction::RIGHT) => Direction::UP,
                    ('/', Direction::DOWN) => Direction::LEFT,
                    ('/', Direction::LEFT) => Direction::DOWN,
                    ('\\', Direction::UP) => Direction::LEFT,
                    ('\\', Direction::LEFT) => Direction::UP,
                    ('\\', Direction::RIGHT) => Direction::DOWN,
                    ('\\', Direction::DOWN) => Direction::RIGHT,
                    ('|', Direction::UP) | ('|', Direction::DOWN) => cart.direction,
                    ('-', Direction::LEFT) | ('-', Direction::RIGHT) => cart.direction,
                    ('+', _) => {
                        let t = cart.turnState;
                        cart.turnState = match FromPrimitive::from_u8(t as u8 + 1) {
                            Some(t) => t,
                            None => FromPrimitive::from_u8(0).unwrap(),
                        };
                        match t {
                            TurnState::LEFT => match cart.direction {
                                Direction::UP => Direction::LEFT,
                                Direction::LEFT => Direction::DOWN,
                                Direction::DOWN => Direction::RIGHT,
                                Direction::RIGHT => Direction::UP,
                            },
                            TurnState::RIGHT => match cart.direction {
                                Direction::UP => Direction::RIGHT,
                                Direction::RIGHT => Direction::DOWN,
                                Direction::DOWN => Direction::LEFT,
                                Direction::LEFT => Direction::UP,
                            },
                            TurnState::STRAIGHT => cart.direction,
                        }
                    }
                    (_, _) => panic!(
                        "Invalid map state '{}': cart={}",
                        self.map[cart.y as usize][cart.x as usize], cart
                    ),
                };
                x = cart.x;
                y = cart.y;
                self.carts[i] = cart;
            }
            // Each time we move a cart, we need to check for collisions.
            let c = match check_collision(&self.carts, i) {
                Some(j) if !clear_collisions => Some(self.carts[min(i, j)].clone()),
                Some(j) => {
                    // Don't advance i in this case...
                    self.carts.remove(j);
                    if j < i {
                        i -= 1;
                    }
                    self.carts.remove(i);
                    None
                }
                None => {
                    i += 1;
                    None
                }
            };
            if c.is_some() {
                return c;
            }
        }
        // No collisions occurred during this update
        None
    }
}

#[aoc_generator(day13)]
pub fn parse(input: &[u8]) -> Map {
    Map::from_u8array(input)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Map) -> Cart {
    let mut map = input.clone();
    let mut c: Option<Cart> = None;
    while c.is_none() {
        c = map.update(false)
        //print!("{}", map);
    }
    c.unwrap()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Map) -> Cart {
    let mut map = input.clone();
    while map.carts.len() > 1 {
        map.update(true);
        //print!("{}", map);
    }
    map.carts[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = b"/->-\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | v  |\n\\-+-/  \\-+--/\n  \\------/   \n";
    const INPUT2: &[u8] = b"/>-<\\  \n|   |  \n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/\n";

    #[test]
    fn part1_example_parse() {
        let map = parse(INPUT);

        let c: &Cart = map.carts.get(0).expect("No cart found");
        assert_eq!(
            c,
            &Cart {
                x: 2,
                y: 0,
                direction: Direction::RIGHT,
                turnState: TurnState::LEFT,
            },
            "Incorrect first cart"
        );
        assert_eq!(
            std::str::from_utf8(INPUT).expect("Invalid UTF8 input"),
            map.get_map(),
            "Invalid input"
        )
    }

    #[test]
    fn part1_example_solve() {
        let mut map = parse(INPUT).clone();

        for i in 0..13 {
            let c = map.update(false);
            println!("{}", map);
            assert_eq!(None, c, "Unexpected collision");
        }

        println!("Crash next...");
        let c = map.update(false);
        println!("{}", map);
        assert_eq!(
            Some(Cart {
                x: 7,
                y: 3,
                direction: Direction::DOWN,
                turnState: TurnState::RIGHT
            }),
            c,
            "Didn't see expected collision"
        )
    }

    #[test]
    fn part2_example_solve() {
        let mut map = parse(INPUT2).clone();
        println!("Original map\n{}", map);

        for i in 0..3 {
            map.update(true);
            println!("{}", map);
        }

        assert_eq!(1, map.carts.len(), "Wrong number of carts left");
        assert_eq!(
            (6, 4),
            (map.carts[0].x, map.carts[0].y),
            "Wrong location for last cart"
        )
    }

    #[test]
    fn test_check_collision() {
        let inputs = &vec![
            (
                vec![
                    Cart::new(0, 0, '>' as u8),
                    Cart::new(0, 1, 'v' as u8),
                    Cart::new(0, 1, '^' as u8),
                ],
                1,
                Some(2),
            ),
            (
                vec![
                    Cart::new(0, 0, '>' as u8),
                    Cart::new(0, 1, 'v' as u8),
                    Cart::new(0, 1, '^' as u8),
                ],
                2,
                Some(1),
            ),
            (
                vec![
                    Cart::new(0, 0, '>' as u8),
                    Cart::new(0, 1, 'v' as u8),
                    Cart::new(0, 2, '^' as u8),
                ],
                1,
                None,
            ),
        ];
        for (i, t) in inputs.iter().enumerate() {
            assert_eq!(check_collision(&t.0, t.1), t.2)
        }
    }
}
