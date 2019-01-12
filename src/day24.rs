use aoc_runner_derive::{aoc,aoc_generator};
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Error;

#[macro_use]
use nom::{IResult,line_ending,is_digit};


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct UnitGroup {
    hp: usize,
    units: usize,
    weak_to: Vec<String>,
    immune_to: Vec<String>,
    damage: usize,
    attack_type: String,
    initiative: usize,
}

impl fmt::Display for UnitGroup {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut ret = format!("{} units each with {} hit points ", self.units, self.hp);
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
        ret.push_str(format!("with an attack that does {} {} damage at initiative {}", self.damage, self.attack_type, self.initiative).as_str());
        writeln!(f, "{}", ret)
    }
}

pub fn parse_unit_group(input: &[u8]) -> IResult<&[u8], UnitGroup> {
    chain!(input,
        units: terminated!(parse_to!(digit, usize), tag!(" units each with ")) ~
        hp: terminated!(parse_to!(digit, usize), tag!(" hit points ")) ~
        weak_immune: opt!(delimited!(
            char!('('),
            separated_list!("; ",
                tuple!(
                    terminated!(alt!(tag!("weak"),tag!("immune")), tag!(" to ")),
                    separated_list!(", ", alpha)
                )
            )
            tag!(") "))) ~
        damage: preceded!(tag!("with an attack that does "), parse_to!(digit, usize)) ~
        attack_type: terminated!(alpha, tag!(" damage"))
        initiative: preceded!(tag!(" at initiative "), parse_to!(digit, usize)) ~
        line_ending,
    || {

        UnitGroup {
            hp,
            units,
            initiative,
            attack_type,
            damage,
        }
    }
    )
}

named!(parse_army<&[u8], (String, &Vec<UnitGroup>)>, chain!(
    army_type: alt!(tag!("Immune System:\n"), tag!("Infection:\n")) >>
    unit_groups: separated_list!("\n", parse_unit_group),
    || {(army_type, unit_groups)}
));

#[aoc_generator(day24)]
fn parse_input(input: &[u8]) -> (Vec<UnitGroup>, Vec<UnitGroup>) {
    let res = parse_army(input);
    println!("{:?}", res);
    res.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Immune System:\n4555 units each with 9688 hit points (immune to radiation; weak to bludgeoning) with an attack that does 17 radiation damage at initiative 1\n";
        parse_input(input.as_bytes());;
    }
}