use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

#[derive(Clone)]
struct State {
    current_recipes: Vec<usize>,
    scoreboard: Vec<usize>,
}

impl State {
    fn new(initial: &Vec<usize>, num_workers: usize) -> State {
        assert!(initial.len() >= num_workers);
        let mut state = State {
            current_recipes: Vec::new(),
            scoreboard: initial.clone(),
        };
        for i in 0..num_workers {
            state.current_recipes.push(i);
        }
        state
    }

    fn make_recipe(&mut self) {
        // Get sum of all current recipes
        let mut score: usize = self.current_recipes.iter().fold(0usize, |acc, i| acc + self.scoreboard[*i]);
        // The number of digits in the score
        let digits = match score {
            0usize => 1i32,
            _ => (score as f64).log10().ceil() as i32 + 1,
        };
        for i in (0..digits).rev() {
            self.scoreboard.push((score / (10f64.powi(i) as usize)) % 10);
        }
        for i in 0..self.current_recipes.len() {
            self.current_recipes[i] = (self.current_recipes[i] + self.scoreboard[i] + 1) % self.scoreboard.len();
        }
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> Option<i32> {
    let mut state= State::new(&vec![3usize, 7], 2);

    let target_scoreboard_num: usize = 681901;

    while state.scoreboard.len() < target_scoreboard_num + 10 {
        state.make_recipe();
    }

    let mut ret= Vec::with_capacity(10);
    ret.copy_from_slice(&state.scoreboard[target_scoreboard_num..target_scoreboard_num+10]);
    println!("{:?}", ret);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut state = State::new(&vec![3usize, 7usize], 2);
        for step in 0..13 {
            state.make_recipe();
            println!("    {:?}", state.current_recipes);
            println!("--> {:?}", state.scoreboard);
        }
        assert_eq!([5,1,5,8,9,1,6,7,7,9], state.scoreboard[9..19])
    }
}