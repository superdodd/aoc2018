use aoc_runner_derive::aoc;
use std::cmp::min;
use std::mem;
use std::cmp::max;

#[derive(Debug, PartialEq)]
enum PathNodeType {
    Static {
        input: String,
        current: Option<String>,
        next: Option<String>,
    },
    Alternatives {
        input: String,
        alternatives: Vec<PathNodeType>,
        alternatives_idx: Option<usize>,
    },
    Subsegments {
        input: String,
        sub_segments: Vec<PathNodeType>,
    }
}

impl PathNodeType {
    /// Given a string, parse the string into a tree-like structure of PathNodeType objects.
    fn parse(input: &str) -> PathNodeType {
        let mut paren_level: usize = 0;
        let mut top_level_alternatives = false;
        let mut is_static_match = true;
        for c in input.chars() {
            match c {
                '|' if paren_level == 0 => {
                    top_level_alternatives = true;
                    is_static_match = false;
                    break;
                }
                '(' => {
                    paren_level += 1;
                    is_static_match = false;
                }
                ')' => paren_level -= 1,
                _ => (),
            }
        }

        if is_static_match {
            return PathNodeType::Static {
                input: input.to_string(),
                current: None,
                next: Some(input.to_string()),
            };
        } else if top_level_alternatives {
            let mut paren_level: usize = 0;
            let children = input.split(|c: char| {
                match c {
                    '|' if paren_level == 0 => true,
                    '(' => {
                        paren_level += 1;
                        false
                    }
                    ')' => {
                        paren_level -= 1;
                        false
                    }
                    _ => false
                }
            }).map(|s: &str| PathNodeType::parse(s)).collect::<Vec<PathNodeType>>();

            PathNodeType::Alternatives {
                input: input.to_string(),
                alternatives: children,
                alternatives_idx: None,
            }
        } else {
            let mut paren_level: usize = 0;
            let mut children = input.split(|c: char| {
                match c {
                    '(' => {
                        paren_level += 1;
                        paren_level == 1
                    }
                    ')' => {
                        paren_level -= 1;
                        paren_level == 0
                    }
                    _ => false
                }
            }).map(|s: &str| PathNodeType::parse(s)).collect::<Vec<PathNodeType>>();
            // In order to get the correct behavior, we need to "prime" the iterator for all
            // but the last sub-segments
            if children.len() > 1 {
                for i in 0_usize..children.len() - 1 {
                    children[i].next();
                }
            }
            PathNodeType::Subsegments {
                input: input.to_string(),
                sub_segments: children,
            }
        }
    }

    /// Resets the state of the iterator to begin iteration again.
    fn reset(&mut self) {
        match self {
            PathNodeType::Static { input, current, next } => {
                *current = None;
                *next = Some(input.to_string());
            }
            PathNodeType::Alternatives { alternatives_idx, alternatives, .. } => {
                *alternatives_idx = None;
                for mut a in alternatives {
                    a.reset();
                }
            }
            PathNodeType::Subsegments { sub_segments, .. } => {
                for i in 0..sub_segments.len() {
                    sub_segments[i].reset();
                    if i < sub_segments.len() - 1 {
                        sub_segments[i].next();
                    }
                }
            }
        }
    }

    /// Returns the "current" value of the segment iterator, or None if the iterator has been
    /// exhausted.  This call will not advance the iterator.
    fn current(&self) -> Option<String> {
        match self {
            PathNodeType::Static { current, .. } => current.clone(),
            PathNodeType::Alternatives { alternatives, alternatives_idx, .. } => {
                if let Some(idx) = *alternatives_idx {
                    if idx >= alternatives.len() {
                        None
                    } else {
                        alternatives[idx].current()
                    }
                } else {
                    None
                }
            },
            PathNodeType::Subsegments { sub_segments, .. } => {
                let mut ret = String::new();
                for s in sub_segments {
                    if let Some(curr) = s.current() {
                        ret.push_str(curr.as_str());
                    } else {
                        return None;
                    }
                }
                Some(ret)
            },
        }
    }

    /// Iterate through all paths, determining the farthest bounds in each direction required.
    /// Reset the iterator afterward.
    fn determine_map_edges(&mut self) -> (i32, i32, i32, i32) {
        let ret = self.map(|p| determine_map_size(&p))
                    .fold((0, 0, 0, 0),
                          |a, r| (min(a.0, r.0), min(a.1, r.1), max(a.2, r.2), max(a.3, r.3)));
        self.reset();
        ret
    }
}

impl Iterator for PathNodeType {
    type Item = String;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self {
            PathNodeType::Static { current, next, .. } => {
                *current = mem::replace(next, None);
            }
            PathNodeType::Alternatives { alternatives, alternatives_idx, .. } => {
                match *alternatives_idx {
                    None => {
                        *alternatives_idx = Some(0);
                        alternatives[0].next();
                    }
                    Some(i) => {
                        let mut j = i;
                        while j < alternatives.len() {
                            match alternatives[j].next() {
                                None => j += 1,
                                Some(_) => break,
                            }
                        }
                        *alternatives_idx = Some(j);
                    }
                }
            }
            PathNodeType::Subsegments { sub_segments, .. } => {
                assert!(sub_segments.len() > 0);
                let mut i: i32 = sub_segments.len() as i32 - 1;
                while sub_segments[i as usize].next().is_none() {
                    if i == 0 {
                        break;
                    }
                    sub_segments[i as usize].reset();
                    sub_segments[i as usize].next();
                    i -= 1;
                }
            }
        }
        self.current()
    }
}

// Given a path, return how far in each direction the path ends up traveling.
fn determine_map_size(path: &str) -> (i32, i32, i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xmin: i32 = 0;
    let mut ymin: i32 = 0;
    let mut xmax: i32 = 0;
    let mut ymax: i32 = 0;
    for step in path.as_bytes().iter() {
        match *step as char {
            'N' => y += 1,
            'E' => x += 1,
            'W' => x -= 1,
            'S' => y -= 1,
            _ => panic!("Bad step: {}", step),
        }
        if x < xmin {
            xmin = x;
        }
        if x > xmax {
            xmax = x;
        }
        if y < ymin {
            ymin = y;
        }
        if y > ymax {
            ymax = y;
        }
    }
    (xmin, ymin, xmax, ymax)
}

fn find_map_edges(paths: &mut PathNodeType) -> (i32, i32, i32, i32) {

    paths.map(|p| determine_map_size(&p))
            .fold((0, 0, 0, 0),
                  |a, r| (min(a.0, r.0), min(a.1, r.1), max(a.2, r.2), max(a.3, r.3)))
}

fn print_map(map: &Vec<Vec<Room>>) {
    for row in map.iter() {
        println!("{}", row.iter().map(|room| match (room.n, room.e, room.s, room.w) {
                (Some(true), Some(true), Some(true), Some(true)) => '┼',
                (Some(true), Some(true), Some(true), Some(false)) => '├',
                (Some(true), Some(true), Some(false), Some(false)) => '└',
                (Some(true), Some(false), Some(false), Some(false)) => '╵',
                (Some(false), Some(true), Some(true), Some(true)) => '┬',
                (Some(true), Some(false), Some(false), Some(true)) => '┘',
                (Some(false), Some(true), Some(false), Some(false)) => '╶',
                (Some(false), Some(false), Some(true), Some(true)) => '┐',
                (Some(false), Some(false), Some(true), Some(false)) => '╷',
                (Some(false), Some(false), Some(false), Some(true)) => '╴',
                (Some(false), Some(false), Some(false), Some(false)) => ' ',
                (Some(true), Some(false), Some(true), Some(true)) => '┤',
                (Some(true), Some(false), Some(true), Some(false)) => '│',
                (Some(false), Some(true), Some(false), Some(true)) => '─',
                (Some(false), Some(true), Some(true), Some(false)) => '┌',
                (Some(true), Some(true), Some(false), Some(true)) => '┴',
                _ => panic!("Invalid map state"),
            }).collect::<String>());
    }
}

#[derive(Default, Clone)]
struct Room {
    n: Option<bool>,
    e: Option<bool>,
    s: Option<bool>,
    w: Option<bool>,
}


//#[aoc(day20, part1, full_parse)]
fn solve_part1(input: &str) -> usize {
    let root = PathNodeType::parse(input);
    root.count()
}

#[aoc(day20, part1, walk)]
fn walk_part1(input: &str) -> usize {
    // This is a stack of where we should go back to when we start an alternate branch.
    let mut backtrack_stack: Vec<(i32, i32)> = Vec::new();

    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut loc= (0, 0);
    for c in input.chars() {
        match c {
            '^' | '$' => (),
            '(' => backtrack_stack.push(loc),
            ')' => loc = backtrack_stack.pop().unwrap(),
            '|' => loc = *backtrack_stack.last().unwrap(),
            'N' => {
                loc.1 -= 1;
                min_y = min(min_y, loc.1);
            }
            'E' => {
                loc.0 += 1;
                max_x = max(max_x, loc.0);
            }
            'W' => {
                loc.0 -= 1;
                min_x = min(min_x, loc.0);
            }
            'S' => {
                loc.1 += 1;
                max_y = max(max_y, loc.1);
            }
            _ => panic!("Invalid character {}", c),
        }
    }
    println!("{:?}", (min_x, min_y, max_x, max_y));
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(PathNodeType::parse("NEW"),
        PathNodeType::Static {
            input: "NEW".to_string(),
            current: None,
            next: Some("NEW".to_string()),
        });
        assert_eq!(PathNodeType::parse("N|E(W|)S"),
        PathNodeType::Alternatives {
            input: "N|E(W|)S".to_string(),
            alternatives_idx: None,
            alternatives: vec![
                PathNodeType::Static {
                    input: "N".to_string(),
                    current: None,
                    next: Some("N".to_string()),
                },
                PathNodeType::Subsegments {
                    input: "E(W|)S".to_string(),
                    sub_segments: vec![
                        PathNodeType::Static {
                            input: "E".to_string(),
                            current: Some("E".to_string()),
                            next: None,
                        },
                        PathNodeType::Alternatives {
                            input: "W|".to_string(),
                            alternatives_idx: Some(0),
                            alternatives: vec![
                                PathNodeType::Static {
                                    input: "W".to_string(),
                                    current: Some("W".to_string()),
                                    next: None,
                                },
                                PathNodeType::Static {
                                    input: "".to_string(),
                                    current: None,
                                    next: Some("".to_string())
                                }
                            ]
                        },
                        PathNodeType::Static {
                            input: "S".to_string(),
                            current: None,
                            next: Some("S".to_string()),
                        },
                    ]
                }
            ]
        })
    }

    #[test]
    fn test_iter() {
        let cases = vec![
            ("N|E|W|S", "N,E,W,S"),
            ("N(E|W|)|S", "NE,NW,N,S"),
            ("N(E|W)", "NE,NW"),
        ];
        for (inp, out) in cases {
            assert_eq!(PathNodeType::parse(inp).collect::<Vec<String>>(), out.split(",").map(|s| s.to_string()).collect::<Vec<String>>());
        }
    }
}
