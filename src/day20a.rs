use aoc_runner_derive::aoc;
use std::cmp::min;
use std::mem;

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
    fn parse(input: &'static str) -> PathNodeType {
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
                        println!("{:#?}", self);
                        println!("{:#?}", s);
                        return None;
                    }
                }
                Some(ret)
            },
        }
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


#[aoc(day20, part1)]
fn solve_part1(input: &str) -> usize {
    let root = PathNodeType::parse(input);
    for i in root
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
