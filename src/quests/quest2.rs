use std::{collections::HashSet, fmt::Error, str::FromStr};

use crate::std::Quest::Quest;

pub struct Quest2 {}

impl Quest for Quest2 {
    fn part1(&self, input: &str) -> String {
        let (possible_words, inscription) = parse_input(input);
        let result: u32 = find_words(&possible_words, &inscription);
        format!("{}", result)
    }

    fn part2(&self, input: &str) -> String {
        let (possible_words, inscription) = parse_input(input);
        let reversed_words: Vec<String> = possible_words
            .clone()
            .into_iter()
            .map(|word| word.chars().rev().collect())
            .collect();
        let inscription_lines: Vec<&str> = (inscription).split("\r\n").collect();
        let result: u32 = inscription_lines
            .into_iter()
            .map(|line| {
                let line = String::from_str(line).unwrap();
                let symbols: HashSet<u32> = runic_symbol_indices(&possible_words, &line)
                    .union(&runic_symbol_indices(&reversed_words, &line))
                    .cloned()
                    .collect();
                symbols.len() as u32
            })
            .sum();
        format!("{}", result)
    }

    fn part3(&self, input: &str) -> String {

        
        String::from_str("0").unwrap()
    }

    fn number(&self) -> u8 {
        2
    }
}

fn parse_input(input: &str) -> (Vec<String>, String) {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let words: Vec<String> = parts
        .get(0)
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(",")
        .into_iter()
        .map(|string| String::from_str(string).unwrap())
        .collect();
    let inscription = String::from_str(parts.get(1).unwrap()).unwrap();
    (words, inscription)
}

fn find_words(possible_words: &Vec<String>, inscription: &String) -> u32 {
    possible_words
        .into_iter()
        .map(|word| inscription.matches(word).count() as u32)
        .sum()
}

fn runic_symbol_indices(possible_words: &Vec<String>, inscription: &String) -> HashSet<u32> {
    let mut indices = HashSet::new();
    for word in possible_words {
        for index in 0..inscription.len() {
            if !match_word_at(word, inscription, index) {
                continue;
            }
            for i in 0..word.len() {
                indices.insert((index + i) as u32);
            }
        }
    }
    indices
}

fn match_word_at(word: &String, inscription: &String, index: usize) -> bool {
    for i in 0..word.len() {
        if word[i..(i + 1)]
            != *match inscription.get((i + index)..(i + index + 1)) {
                Some(character) => character,
                None => return false,
            }
        {
            return false;
        }
    }
    true
}
