use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
struct UnitGroup {
    group_number: usize,
    hp: usize,
    units: usize,
    weak_to: Vec<String>,
    immune_to: Vec<String>,
    damage: usize,
    attack_type: String,
    initiative: usize,
}

type Army = ArmyType<UnitGroup>;

struct ArmyType<T>(Vec<T>)
where
    T: Ord;

impl fmt::Display for UnitGroup {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut ret = format!(
            "Group {}: {} units each with {} hit points ",
            self.group_number, self.units, self.hp
        );
        if !self.weak_to.is_empty() || !self.immune_to.is_empty() {
            ret.push('(');
            if !self.weak_to.is_empty() {
                ret.push_str("weak to ");
                ret.push_str(self.weak_to.join(", ").as_str());
            }
            if !self.weak_to.is_empty() && !self.immune_to.is_empty() {
                ret.push_str("; ");
            }
            if !self.immune_to.is_empty() {
                ret.push_str("immune to ");
                ret.push_str(self.immune_to.join(", ").as_str());
            }
            ret.push_str(") ")
        }
        ret.push_str(
            format!(
                "with an attack that does {} {} damage at initiative {}",
                self.damage, self.attack_type, self.initiative
            )
            .as_str(),
        );
        writeln!(f, "{}", ret)
    }
}

impl UnitGroup {
    fn parse(input: &str, group_number: usize) -> UnitGroup {
        let line_re = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points (?P<mods>\((?:(?:weak|immune) to (?:[^;)]+)(?:; )?)+\))? with an attack that does (?P<dmg>\d+) (?P<type>[^\s]+) damage at initiative (?P<init>\d+)").unwrap();
        let captures = line_re.captures(input).unwrap();
        let mut weak_to: Vec<String> = Vec::new();
        let mut immune_to: Vec<String> = Vec::new();
        if let Some(mods) = captures.name("mods") {
            let mod_tuples = mods.as_str()[1..&mods.as_str().len() - 1]
                .split("; ")
                .flat_map(|m: &str| {
                    let words: Vec<&str> = m.split(" ").collect();
                    let mut ret: Vec<(String, String)> = Vec::new();
                    for w in 2..words.len() {
                        ret.push((
                            words[0].to_owned(),
                            words[w].trim_end_matches(",").to_owned(),
                        ));
                    }
                    ret.into_iter()
                })
                .collect::<Vec<(String, String)>>();
            for (wi, t) in mod_tuples {
                match wi.as_str() {
                    "weak" => weak_to.push(t),
                    "immune" => immune_to.push(t),
                    _ => panic!("Unknown keyword {}", wi),
                }
            }
        }

        UnitGroup {
            group_number,
            weak_to,
            immune_to,
            hp: captures.name("hp").unwrap().as_str().parse().unwrap(),
            units: captures.name("units").unwrap().as_str().parse().unwrap(),
            damage: captures.name("dmg").unwrap().as_str().parse().unwrap(),
            attack_type: captures.name("type").unwrap().as_str().to_owned(),
            initiative: captures.name("init").unwrap().as_str().parse().unwrap(),
        }
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> (Army, Army) {
    let mut infection: Vec<UnitGroup> = Vec::new();
    let mut immune_system: Vec<UnitGroup> = Vec::new();

    let mut current_army = String::new();
    let mut unit_group_number: usize = 1;

    for l in input.lines().map(|l| l.trim_left()) {
        match l {
            "" => (),
            "Immune System:" | "Infection:" => {
                current_army = l.to_string();
                unit_group_number = 1;
            }
            _ => {
                match current_army.as_str() {
                    "Immune System:" => immune_system.push(UnitGroup::parse(l, unit_group_number)),
                    "Infection:" => infection.push(UnitGroup::parse(l, unit_group_number)),
                    _ => panic!("Unknown army type {}", current_army),
                }
                unit_group_number += 1;
            }
        }
    }

    (immune_system, infection)
}

fn fight(immune_system: &mut Army, infection: &mut Army) {}

#[aoc(day24, part1)]
fn solve_part1(input: &(Army, Army)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "4555 units each with 9688 hit points (immune to fire; weak to slashing, bludgeoning) with an attack that does 17 radiation damage at initiative 1\n";
        let res = UnitGroup::parse(input, 1);
        assert_eq!(
            res,
            UnitGroup {
                group_number: 1,
                hp: 9688,
                units: 4555,
                weak_to: vec!["slashing".to_string(), "bludgeoning".to_string()],
                immune_to: vec!["fire".to_string()],
                damage: 17,
                attack_type: "radiation".to_string(),
                initiative: 1
            }
        );
    }

    #[test]
    fn test_part1() {
        let input = "
        Immune System:
        17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
        989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

        Infection:
        801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
        4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
        ";
        let (mut immune_system, mut infection) = parse_input(input);

        assert_eq!(2, immune_system.len());
        assert_eq!(2, infection.len());
    }

    #[test]
    fn test_fight() {
        let input = "
        Immune System:
        17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
        989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

        Infection:
        801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
        4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
        ";
        let (mut immune_system, mut infection) = parse_input(input);

        let units_list = |l: &Vec<UnitGroup>| {
            l.iter()
                .map(|ug| (ug.group_number, ug.units))
                .collect::<Vec<(usize, usize)>>()
        };

        assert_eq!(vec![(1, 17), (2, 989)], units_list(&immune_system));
        assert_eq!(vec![(1, 801), (2, 4485)], units_list(&infection));

        fight(&mut immune_system, &mut infection);

        assert_eq!(
            vec![(2, 905)],
            units_list(&immune_system),
            "Immune system unit count"
        );
        assert_eq!(
            vec![(1, 797), (2, 4434)],
            units_list(&infection),
            "Infection unit count"
        );
    }
}
