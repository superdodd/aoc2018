use aoc_runner_derive::{aoc_generator, aoc};
use std::cmp::min;
use std::cmp::max;

#[aoc_generator(day20)]
fn generate_path_tree(input: &str) -> PathTreeNode {
    let mut idx: usize = 0;
    return PathTreeNode::parse_from(input);
}

struct PathTreeNode<'a> {
    content: &'a str,
    children: &'a [PathTreeNode],
}

impl PathTreeNode {
    fn parse_from(input: &str) -> PathTreeNode {
        if input.is_empty() {
            return PathTreeNode {
                content: "",
                children: &[],
            };
        }
        let open_paren = input.find('(').unwrap_or(input.len());
        let split = input.find('|').unwrap_or(input.len());
        if split < open_paren {
            // If we hit a split character before any open parens, then we have one child that
            // starts after the first close-paren we encounter.
            let close_paren = &input[split..].find(')').unwrap_or(input.len());
            return PathTreeNode {
                content: &input[0..split],
                children: &[PathTreeNode::parse_from(&input[close_paren + 1..])],
            };
        } else if open_paren < split {
            // If we hit an open paren first, we need to create one child that starts after each
            // split character within the top level of parens.
            let close_paren = &input[open_paren..].find(')').unwrap_or(input.len());
            let mut paren_level: usize = 0;
            let mut children: Vec<PathTreeNode> = Vec::new();
            for i in open_paren + 1..close_paren {
                match input[i] {
                    '|' => children.push(PathTreeNode::parse_from(&input[i + 1..])),
                    '(' => paren_level += 1,
                    ')' if paren_level == 0 => break,
                    ')' => paren_level -= 1,
                    _ => (),
                }
            }
            return PathTreeNode {
                content: &input[0..open_paren],
                children: children.as_slice(),
            }
        }
        // Otherwise, return the rest of the content as a single node.
        return PathTreeNode{
            content: input,
            children: &[],
        }
    }
}

fn parse_all_paths(input: &str) -> Vec<String> {
    let mut to_expand: Vec<String> = vec![input.to_owned()];
    let mut ret: Vec<String> = Vec::new();

    while !to_expand.is_empty() {
        let item = to_expand.pop().unwrap();
        // Find an innermost set of parentheses and expand all variants within it.
        if let Some(close_idx) = item.find(')') {
            if let Some(open_idx) = item[0..close_idx].rfind('(') {
                for segment in item[open_idx + 1..close_idx].split('|') {
                    let mut path_repl = String::with_capacity(open_idx + segment.len() + item.len() - close_idx);
                    path_repl.push_str(&item[0..open_idx]);
                    path_repl.push_str(segment);
                    path_repl.push_str(&item[close_idx + 1..]);
                    to_expand.push(path_repl.to_owned());
                }
            } else {
                panic!("Paren mismatch, no open found for idx {}: {}", close_idx, item);
            }
        } else {
            ret.push(item);
        }
    }
    ret.sort();
    ret.dedup();
    ret
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

fn find_map_edges(paths: &Vec<String>) -> (i32, i32, i32, i32) {
    paths.iter().map(|p| determine_map_size(p))
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
fn solve_part1(paths: &Vec<String>) -> usize {
    // First, determine how big our map should be.
    let maprange = find_map_edges(paths);

    let mut map: Vec<Vec<Room>> = Vec::with_capacity((maprange.3 - maprange.1 + 3) as usize);
    for _ in 0..map.capacity() {
        map.push(vec![Room::default(); (maprange.2 - maprange.0 + 3) as usize]);
    }

    let xstart = -maprange.0 as usize;
    let ystart = -maprange.1 as usize;
    // Trace out each path to fill in the map.
    for path in paths {
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
        let mut output = parse_all_paths(input);
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
        assert_eq!((-2, 0, 1, 1), find_map_edges(&paths));
    }

}