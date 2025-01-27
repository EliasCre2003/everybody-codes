use std::{fs::File, io::{Error, Read}};

pub mod Quest;

pub mod Grid;

pub fn get_inputs(quest_number: u8, test: bool) -> Result<(String, String, String), Error> {
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
    Ok((
        inputs.get(0).unwrap().clone(),
        inputs.get(1).unwrap().clone(),
        inputs.get(2).unwrap().clone(),
    ))
}
