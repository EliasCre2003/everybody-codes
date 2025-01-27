
use std::str::FromStr;

use crate::std::Quest::Quest;

pub struct Quest10 {}

impl Quest for Quest10 {
    fn part1(&self, input: &str) -> String {
        String::from_str("0").unwrap()
    }

    fn part2(&self, input: &str) -> String {
        String::from_str("0").unwrap()
    }

    fn part3(&self, input: &str) -> String {
        String::from_str("0").unwrap()
    }

    fn number(&self) -> u8 {
        10
    }
}
