use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str;
use num_traits::abs;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
struct UnitGroup {
    army: String,
    group_number: usize,
    hp: usize,
    units: usize,
    weak_to: Vec<String>,
    immune_to: Vec<String>,
    damage: usize,
    attack_type: String,
    initiative: usize,
}

impl Ord for UnitGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare_by_elem = |a: &Vec<String>, b: &Vec<String>| {
            a.iter()
                .zip(b.iter())
                .find(|&(s, o)| s != o)
                .map(|(s, o)| s.cmp(o))
                .unwrap_or(Ordering::Equal)
        };

        (self.damage * self.units)
            .cmp(&(other.damage * other.units))
            .then(self.initiative.cmp(&other.initiative))
            .then(self.group_number.cmp(&other.group_number))
            .then(self.damage.cmp(&other.damage))
            .then(self.hp.cmp(&other.hp))
            .then(compare_by_elem(&self.weak_to, &other.weak_to))
            .then(compare_by_elem(&self.immune_to, &other.immune_to))
            .then(self.attack_type.cmp(&other.attack_type))
            .then(self.army.cmp(&other.army))
    }
}

impl PartialOrd for UnitGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl fmt::Display for UnitGroup {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut ret = format!(
            "{} Group {}: {} units each with {} hit points ",
            self.army, self.group_number, self.units, self.hp
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
    fn parse(input: &str, army: &str, group_number: usize) -> UnitGroup {
        let line_re = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points (?P<mods>\((?:(?:weak|immune) to (?:[^;)]+)(?:; )?)+\) )?with an attack that does (?P<dmg>\d+) (?P<type>[^\s]+) damage at initiative (?P<init>\d+)").unwrap();
        let captures = line_re.captures(input).unwrap();
        let mut weak_to: Vec<String> = Vec::new();
        let mut immune_to: Vec<String> = Vec::new();
        if let Some(mods) = captures.name("mods") {
            let mod_tuples = mods.as_str()[1..&mods.as_str().len() - 2]
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

        let get_captured_usize =
            |f: &str| captures.name(f).unwrap().as_str().parse::<usize>().unwrap();
        let get_captured_string = |f: &str| captures.name(f).unwrap().as_str().to_owned();

        UnitGroup {
            army: army.to_string(),
            group_number,
            weak_to,
            immune_to,
            hp: get_captured_usize("hp"),
            units: get_captured_usize("units"),
            damage: get_captured_usize("dmg"),
            attack_type: get_captured_string("type"),
            initiative: get_captured_usize("init"),
        }
    }

    fn damage_to(&self, other: &UnitGroup) -> usize {
        if other.immune_to.contains(&self.attack_type) {
            return 0;
        }
        if other.weak_to.contains(&self.attack_type) {
            return 2 * self.damage * self.units;
        }
        return self.damage * self.units;
    }

    fn attack(&self, other: &mut UnitGroup) {
        let damage = self.damage_to(other);
        let units_killed = damage / other.hp;
        if units_killed >= other.units {
            other.units = 0;
        } else {
            other.units -= units_killed;
        }
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<UnitGroup> {
    let mut units: Vec<UnitGroup> = Vec::new();

    let mut current_army = String::new();
    let mut unit_group_number: usize = 1;

    for l in input.lines().map(|l| l.trim_left()) {
        match l {
            "" => (),
            "Immune System:" | "Infection:" => {
                current_army = l.to_string();
                unit_group_number = units
                    .iter()
                    .filter(|&u| u.army == l)
                    .max_by(|&a, &b| a.group_number.cmp(&b.group_number))
                    .map(|u| u.group_number)
                    .unwrap_or(1);
            }
            _ => {
                units.push(UnitGroup::parse(l, &current_army, unit_group_number));
                unit_group_number += 1;
            }
        }
    }

    units
}

fn fight(units: &mut Vec<UnitGroup>) {
    // First ensure unit list is sorted by effective power / initiative
    units.sort();

    // Map units to their targets.  A value of j in this vector at position i indicates that the
    // unit at position i in the units vector is attacking the unit at position j.
    let mut target_list: Vec<Option<usize>> = vec![None; units.len()];

    // From highest to lowest priority (from the end of the list to the beginning), select a
    // target for each unit.
    for i in (0..units.len()).rev() {
        let attacker = &units[i];
        target_list[i] = units
            .iter()
            .enumerate()
            .filter(|&(i, u)| {
                u.army != attacker.army
                    && !u.immune_to.contains(&attacker.attack_type)
                    && !target_list.contains(&Some(i))
            })
            .max_by(|a, b| attacker.damage_to(a.1).cmp(&attacker.damage_to(b.1)))
            .map(|(i, _u)| i);
    }

    // Units attack in initiative order, highest to lowest.
    let max_initiative = units.iter().map(|u| u.initiative).max().unwrap();
    for initiative in (0..=max_initiative).rev() {
        if let Some((attacker_idx, attacker)) = units
            .iter()
            .enumerate()
            .find(|&(_i, u)| u.initiative == initiative)
        {
            if let Some(defender_idx) = target_list[attacker_idx] {
                if attacker.units > 0 {
                    // Only attack if we didn't already get eliminated this round
                    let mut defender = units[defender_idx].clone();
                    attacker.attack(&mut defender);
                    units[defender_idx] = defender;
                }
            }
        }
    }

    // Clean up the units list, removing any groups that have hp = 0.
    units.retain(|u| u.units > 0);

    // Ensure the units list is still sorted; as groups take damage their effective power decreases.
    units.sort();
}

#[aoc(day24, part1)]
fn solve_part1(input: &Vec<UnitGroup>) -> usize {
    let mut units = input.to_vec();

    while {
        let cnt = units.iter().filter(|&u| u.army == "Infection:").count();
        cnt > 0 && cnt < units.len()
    } {
        fight(&mut units);
    }
    units.iter().map(|u| u.units).sum()
}

#[aoc(day24, part2)]
fn solve_part2(input: &Vec<UnitGroup>) -> usize {

    let mut lower_bound: usize = 0;
    let mut upper_bound: usize = input.iter().map(|u| u.hp * u.units).max().unwrap_or(1);
    let mut upper_bound_units: usize = 0;

    while upper_bound - lower_bound > 1 {
        let boost = (upper_bound + lower_bound) / 2;
        let mut units = input.clone();
        for u in units.iter_mut() {
            u.damage += boost;
        }

        while {
            let cnt = units.iter().filter(|u| u.army == "Infection:").count();
            cnt > 0 && cnt < units.len()
        } {
            fight(&mut units);
        }

        if units.iter().find(|&u| u.army == "Immune System:").is_some() {
            upper_bound = boost;
            upper_bound_units = units.iter().map(|u| u.units).sum();
        } else {
            lower_bound = boost;
        }
    }

    upper_bound_units
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "4555 units each with 9688 hit points (immune to fire; weak to slashing, bludgeoning) with an attack that does 17 radiation damage at initiative 1\n";
        let res = UnitGroup::parse(input, "Immune System:", 1);
        assert_eq!(
            res,
            UnitGroup {
                army: "Immune System:".to_string(),
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
        4485 units each with 2961 hit points with an attack that does 12 slashing damage at initiative 4
        ";
        let units = parse_input(input);

        assert_eq!(4, units.len());
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
        let mut units = parse_input(input);

        units.sort();

        let units_list = |l: &Vec<UnitGroup>| {
            l.iter()
                .map(|ug| (ug.army[0..3].to_string(), ug.group_number, ug.units))
                .collect::<Vec<(String, usize, usize)>>()
        };

        assert_eq!(
            vec![
                ("Imm".to_string(), 2, 989),
                ("Inf".to_string(), 2, 4485),
                ("Imm".to_string(), 1, 17),
                ("Inf".to_string(), 1, 801)
            ],
            units_list(&units)
        );

        fight(&mut units);
        assert_eq!(
            vec![
                ("Imm".to_string(), 2, 905),
                ("Inf".to_string(), 2, 4434),
                ("Inf".to_string(), 1, 797)
            ],
            units_list(&units)
        );

        fight(&mut units);
        assert_eq!(
            vec![
                ("Imm".to_string(), 2, 761),
                ("Inf".to_string(), 2, 4434),
                ("Inf".to_string(), 1, 793)
            ],
            units_list(&units)
        );

        for _ in 0..6 {
            fight(&mut units);
        }

        assert_eq!(
            vec![("Inf".to_string(), 2, 4434), ("Inf".to_string(), 1, 782)],
            units_list(&units)
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = "
        Immune System:
        17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
        989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

        Infection:
        801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
        4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
        ";

        let units = parse_input(input);
        assert_eq!(5216, solve_part1(&units))
    }
}
