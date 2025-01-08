use everybody_codes::quests::quest1::Quest1;
use everybody_codes::std::{get_inputs, Quest};

fn main() {
    let test = false;
    let quest = Quest1 {};
    let inputs = get_inputs(quest.number(), test).unwrap();
    let result = quest.all(inputs);
    println!(
        "Part 1: {}\nPart 2: {}\nPart 3: {}",
        result.get(0).unwrap(),
        result.get(1).unwrap(),
        result.get(2).unwrap()
    );
}
