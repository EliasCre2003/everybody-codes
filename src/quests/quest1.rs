use crate::std::Quest::Quest;

pub struct Quest1 {}

impl Quest for Quest1 {
    fn part1(&self, input: &str) -> String {
        let result = total_required_potions(input, 1);
        format!("{}", result)
    }

    fn part2(&self, input: &str) -> String {
        let result = total_required_potions(input, 2);
        format!("{}", result)
    }

    fn part3(&self, input: &str) -> String {
        let result = total_required_potions(input, 3);
        format!("{}", result)
    }

    fn number(&self) -> u8 {
        1
    }
}

fn required_potions(monster: &char) -> u32 {
    match monster {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn is_monster(monster: &char) -> bool {
    "ABCD".contains(*monster)
}

fn total_required_potions(notes: &str, group_size: usize) -> u32 {
    let monsters: Vec<char> = notes.chars().collect();
    (0..monsters.len())
        .step_by(group_size)
        .into_iter()
        .map(|index| {
            let mut total = 0;
            let n_monsters: u32 = (0..group_size)
                .into_iter()
                .map(|i| {
                    let monster = monsters.get(index + i).unwrap();
                    total += required_potions(monster);
                    is_monster(monster) as u32
                })
                .sum();
            total
                + match n_monsters {
                    2 => 2,
                    3 => 6,
                    _ => 0,
                }
        })
        .sum()
}
