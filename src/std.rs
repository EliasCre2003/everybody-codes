use std::{
    fs::File,
    io::{Error, Read},
    vec,
};

pub trait Quest {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
    fn part3(&self, input: &str) -> String;
    fn all(&self, inputs: Vec<String>) -> Vec<String> {
        vec![
            self.part1(&inputs.get(0).unwrap()),
            self.part2(&inputs.get(1).unwrap()),
            self.part3(&inputs.get(2).unwrap()),
        ]
    }
    fn number(&self) -> u8;
}

pub fn get_inputs(quest_number: u8, test: bool) -> Result<Vec<String>, Error> {
    let mut inputs = Vec::new();
    let file_path = format!(
        "inputs/quest{}/{}/",
        quest_number,
        if test { "tests" } else { "inputs" }
    );
    for index in 0..3 {
        let file_path = format!("{}part{}.txt", file_path, index + 1);
        let mut input = String::new();
        match match File::open(&file_path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        }
        .read_to_string(&mut input)
        {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        inputs.push(input);
    }
    Ok(inputs)
}
