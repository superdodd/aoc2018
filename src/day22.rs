use aoc_runner_derive::aoc;

const DEPTH: usize = 11820;
const TARGET: (usize, usize) = (7, 782);

fn get_map(depth: usize, target: (usize, usize), limit: (usize, usize)) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![0; limit.0 + 1]; limit.1 + 1];
    for y in 0..=limit.1 {
        let mut row = &ret[y];
        for x in 0..=limit.0 {
            let geo_idx: usize = if (x, y) == target {
                0
            } else {
                match (x, y) {
                    (0, 0) => 0,
                    (x, 0) => x * 16807,
                    (0, y) => y * 48271,
                    (x, y) => ret[y][x-1] * ret[y-1][x],
                }
            };
            ret[y][x] = (geo_idx + depth) % 20183;
        }
    }
    ret
}

#[aoc(day22, part1)]
fn solve_part1(_input: &str) -> usize {
    let map = get_map(DEPTH, TARGET, TARGET);
    let mut risk_level: usize = 0;
    for row in &map {
        for lvl in row {
            risk_level += (*lvl % 3);
        }
    }
    risk_level
}

#[aoc(day22, part2)]
fn solve_part2(_input: &str) -> usize {
    let map = get_map(DEPTH, TARGET, TARGET);


}