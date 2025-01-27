use everybody_codes::quests::{
    quest1::Quest1,
    quest2::Quest2
};
use everybody_codes::std::Quest::Quest;
use everybody_codes::std::get_inputs;

fn main() {
    let test = false;
    let quest = Quest2 {};
    let inputs = get_inputs(quest.number(), test).unwrap();
    let result = quest.all(inputs);
    println!(
        "Part 1: {}\nPart 2: {}\nPart 3: {}",
        result.0,
        result.1,
        result.2
    );
}
