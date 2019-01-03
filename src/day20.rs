use aoc_runner_derive::aoc;
use std::cmp::min;
use std::cmp::max;
use std::rc::Rc;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Default, Clone, Debug)]
struct PathIterator {
    // A reference to the original entire pattern
    original_input: Rc<String>,
    // The start and end of this segment
    start: usize,
    end: usize,
    // If this segment has sub-patterns, this represents each.
    sub_segments: Vec<PathIterator>,
    // Otherwise this segment matches a set of mutually exclusive alternatives.
    pattern_alternatives: Vec<PathIterator>,
    pattern_alternatives_idx: usize,
    // Store the most recently returned value for future reference.
    current_pattern: Option<String>,
}

impl PathIterator {
    fn new(original_input: Rc<String>, start: usize, end: usize) -> PathIterator {

        let mut path_iterator = PathIterator{
            original_input,
            start,
            end,
            ..PathIterator::default()
        };
        path_iterator.compute_alternatives();
        if path_iterator.pattern_alternatives.is_empty() {
            path_iterator.compute_sub_segments();
            if path_iterator.sub_segments.is_empty() {
                path_iterator.current_pattern = Some(path_iterator.original_input[start..end].to_string());
            }
        }

        path_iterator
    }

    fn reset(&mut self) {
        if !self.pattern_alternatives.is_empty() {
            self.pattern_alternatives_idx = 0;
        } else if !self.sub_segments.is_empty() {
            for s in &mut self.sub_segments {
                s.reset();
            }
        } else {
            self.current_pattern = Some(self.original_input[self.start..self.end].to_string());
        }
    }

    fn compute_alternatives(&mut self) {
        let input = &self.original_input.clone()[self.start..self.end];
        let mut paren_level: usize = 0;
        let mut segment_start: usize = 0;
        let mut alternatives = input.chars().enumerate().filter_map(|(i, c)| match c {
            '|' if paren_level == 0 => {
                let ret = PathIterator::new(self.original_input.clone(), self.start + segment_start, self.start + i);
                segment_start = i + 1;
                Some(ret)
            }
            '(' => {
                paren_level += 1;
                None
            }
            ')' => {
                paren_level -= 1;
                None
            }
            _ => None,
        }).collect::<Vec<PathIterator>>();
        if !alternatives.is_empty()  {
            // Make sure we include the last segment
            alternatives.push(PathIterator::new(self.original_input.clone(), self.start + segment_start, self.start + input.len()));
            self.pattern_alternatives = alternatives;
        }
    }

    fn compute_sub_segments(&mut self) {
        let input = &self.original_input[self.start..self.end];
        let mut segment_start: usize = 0;
        let mut paren_level: usize = 0;
        let mut sub_segments= input.chars().enumerate().filter_map(|(i, c)| match c {
            '(' if paren_level > 0 => {
                paren_level += 1;
                None
            }
            '(' => {
                let ret = Some(PathIterator::new(self.original_input.clone(), self.start + segment_start, self.start + i));
                paren_level += 1;
                segment_start = i + 1;
                ret
            }
            ')' => {
                paren_level -= 1;
                if paren_level == 0 {
                    let ret = Some(PathIterator::new(self.original_input.clone(), self.start + segment_start, self.start + i));
                    segment_start = i + 1;
                    ret
                } else {
                    None
                }
            }
            _ => None,
        }).collect::<Vec<PathIterator>>();
        // Make sure we include the last segment
        if !sub_segments.is_empty() {
            sub_segments.push(PathIterator::new(self.original_input.clone(), self.start + segment_start, self.start + input.len()));
            self.sub_segments = sub_segments;
        }
    }
}

impl Iterator for PathIterator {
    type Item = String;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // If this segment is a set of alternatives, return the next pattern from the alternatives.
        if !self.pattern_alternatives.is_empty() {
            loop {
                // When alternatives are exhausted, we return None and let
                // the parent iterator recreate us
                if self.pattern_alternatives_idx >= self.pattern_alternatives.len() {
                    return None;
                }

                match self.pattern_alternatives[self.pattern_alternatives_idx].next() {
                    Some(item) => {
                        self.current_pattern = Some(item);
                        break;
                    }
                    None => self.pattern_alternatives_idx += 1,
                }
            }
            return (&self.current_pattern).clone();
        } else if !self.sub_segments.is_empty() {
            // Otherwise, construct the matching string from our sub-segments
            let mut i: usize = self.sub_segments.len() - 1;
            while self.sub_segments[i].next().is_none() {
                if i == 0 {
                    // Exhausted all options for all sub-segments
                    return None;
                }
                i -= 1;
            }
            // Re-initialize the exhausted sub-segments.
            // Iterate each once so we can peek at the first item.
            for i in i + 1..self.sub_segments.len() {
                self.sub_segments[i].reset();
            }
            let mut ret: String = String::new();
            for s in self.sub_segments.iter() {
                ret.push_str(s.current_pattern.as_ref().unwrap().as_str());
            }
            self.current_pattern = Some(ret);
            return (&self.current_pattern).clone();
        } else {
            self.current_pattern = None;
            return None;
        }
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
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
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

fn find_map_edges(paths: PathIterator) -> (i32, i32, i32, i32) {

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

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> usize {
    let root = PathIterator::new(Rc::new(input.to_string()), 0, input.len());
    
    // First, determine how big our map should be.
    let maprange = find_map_edges(root.clone());

    let mut map: Vec<Vec<Room>> = Vec::with_capacity((maprange.3 - maprange.1 + 3) as usize);
    for _ in 0..map.capacity() {
        map.push(vec![Room::default(); (maprange.2 - maprange.0 + 3) as usize]);
    }

    let xstart = -maprange.0 as usize;
    let ystart = -maprange.1 as usize;
    // Trace out each path to fill in the map.
    for path in root {
        let mut x = xstart;
        let mut y = ystart;
        for &c in path.as_bytes().iter() {
            match c as char {
                'N' => {
                    assert_ne!(Some(false), map[y][x].n);
                    assert_ne!(Some(false), map[y+1][x].s);
                    map[y][x].n = Some(true);
                    map[y+1][x].s = Some(true);
                    y += 1;
                }
                'E' => {
                    assert_ne!(Some(false), map[y][x].e);
                    assert_ne!(Some(false), map[y][x+1].w);
                    map[y][x].e = Some(true);
                    map[y][x+1].w = Some(true);
                    x += 1;
                }
                'S' => {
                    assert_ne!(Some(false), map[y][x].s);
                    assert_ne!(Some(false), map[y-1][x].n);
                    map[y][x].s = Some(true);
                    map[y-1][x].n = Some(true);
                    y -= 1;
                }
                'W' => {
                    assert_ne!(Some(false), map[y][x].w);
                    assert_ne!(Some(false), map[y][x-1].e);
                    map[y][x].w = Some(true);
                    map[y][x-1].e = Some(true);
                    x -= 1;
                }
                _ => panic!("Invalid path: {}", c),
            }
        }
    }

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let mut room = map[r][c].clone();
            if room.n.is_none() {
                room.n = Some(false);
            }
            if room.e.is_none() {
                room.e = Some(false);
            }
            if room.s.is_none() {
                room.s = Some(false);
            }
            if room.w.is_none() {
                room.w = Some(false);
            }
            map[r][c] = room;
        }
    }

    print_map(&map);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "N(E|S(W|)N(E|S))W";
        let mut expected = vec![
            "NEW",
            "NSWNEW",
            "NSWNSW",
            "NSNEW",
            "NSNSW"
        ];
        let mut output: Vec<String> = Vec::new();
        let mut root = PathIterator::new(Rc::new(input.to_string()), 0, input.len());
        println!("{:#?}", root);
        for i in 0..5 {
            println!("--- {} ---", i);
            println!("{:?}", root.next());
            println!("{:#?}", root);
        }
        for i in root {
        }
        output.sort();
        expected.sort();
        assert_eq!(expected, output);
    }

    #[test]
    fn test_map_dimensions() {
        let paths = vec![
                    "NEW",
                    "NSWNEW",
                    "NSWNSW",
                    "NSNEW",
                    "NSNSW"
                ].iter().map(|&s| String::from(s)).collect::<Vec<String>>();
        let expected: Vec<(i32, i32, i32, i32)> = vec![
            (0,0,1,1),
            (-1, 0, 0, 1),
            (-2, 0, 0, 1),
            (0, 0, 1, 1),
            (-1, 0, 0, 1),
        ];
        for (p, &e) in paths.iter().zip(expected.iter()) {
            assert_eq!(e, determine_map_size(&p), "Bad bounds for {}", p);
        }
    }

}