use std::{
    fs::File,
    io::{Error, Read},
    vec,
};

pub trait Quest {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
    fn part3(&self, input: &str) -> String;
    fn all(&self, inputs: (String, String, String)) -> (String, String, String) {
        (
            self.part1(&inputs.0),
            self.part2(&inputs.1),
            self.part3(&inputs.2),
        )
    }
    fn number(&self) -> u8;
}
