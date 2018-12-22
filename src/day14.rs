use aoc_runner_derive::aoc;

#[derive(Clone)]
struct State {
    current_recipes: Vec<usize>,
    scoreboard: Vec<usize>,
}

fn number_to_digits(num: usize) -> Vec<usize> {
    if num == 0 {
        return vec![0];
    }
    let mut ret = Vec::with_capacity((num as f64).log10() as usize + 1);
    let mut val = num;
    while val > 0 {
        ret.insert(0, val % 10);
        val /= 10;
    }
    ret
}

impl State {
    fn new(initial: &[usize], num_workers: usize) -> State {
        assert!(initial.len() >= num_workers);
        let mut state = State {
            current_recipes: Vec::new(),
            scoreboard: initial.to_owned(),
        };
        for i in 0..num_workers {
            state.current_recipes.push(i);
        }
        state
    }

    fn make_recipe(&mut self) {
        // Get sum of all current recipes
        let score: usize = self.current_recipes.iter()
            .fold(0usize, |acc, i| acc + self.scoreboard[*i]);
        // Add the digits of the sum to the scoreboard
        self.scoreboard.extend(number_to_digits(score));

        // Advance each player the correct number of recipes
        for i in 0..self.current_recipes.len() {
            self.current_recipes[i] = (self.current_recipes[i] + self.scoreboard[self.current_recipes[i]] + 1) % self.scoreboard.len();
        }
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(_input: &str) -> String {
    let mut state= State::new(&[3usize, 7], 2);

    let target_scoreboard_num: usize = 681_901;

    while state.scoreboard.len() < target_scoreboard_num + 10 {
        state.make_recipe();
    }

    String::from_utf8(
        state.scoreboard[target_scoreboard_num..target_scoreboard_num+10]
            .iter().map(|c| (*c + '0' as usize) as u8).collect()).expect("Invalid UTF8!")
}

fn find_index_of_slice(match_slice: &[usize]) -> usize {
    let mut state = State::new(&[3usize, 7], 2);

    let mut i: usize = 0;
    loop {
        while state.scoreboard.len() <= i + match_slice.len() {
            state.make_recipe();
        }
        if &state.scoreboard[i..i+match_slice.len()] == match_slice {
            return i;
        } else {
            i += 1;
        }
    }
}

#[aoc(day14, part2)]
fn solve_part2(_input: &str) -> usize {
    find_index_of_slice(&[6usize, 8, 1, 9, 0, 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut state = State::new(&vec![3usize, 7usize], 2);
        for _i in 0..14 {
            state.make_recipe();
            for (i, s) in state.scoreboard.iter().enumerate() {
                if state.current_recipes[0] == i {
                    print!("[{}]", s);
                } else if state.current_recipes[1] == i {
                    print!("({})", s);
                } else if state.current_recipes[2..].contains(&i) {
                    print!("{}{} ", (i as u8 - 2 + 'a' as u8) as char, s);
                } else {
                    print!(" {} ", s)
                }
            }
            println!();
        }
        assert_eq!([5,1,5,8,9,1,6,7,7,9], state.scoreboard[9..19])
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(9, find_index_of_slice(&[5, 1, 5, 8, 9]));
        assert_eq!(5, find_index_of_slice(&[0, 1, 2, 4, 5]));
        assert_eq!(18, find_index_of_slice(&[9, 2, 5, 1, 0]));
        assert_eq!(2018, find_index_of_slice(&[5, 9, 4, 1, 4]));

    }
}