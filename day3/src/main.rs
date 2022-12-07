use std::{collections::BTreeSet, fs::File, io::Read};

use commons::NEWLINE;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut file = File::open("inputs/day3_input.txt")?;
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer)?;
    let raw_str = String::from_utf8(buffer)?;

    let sum_of_priorities = raw_str
        .split(NEWLINE)
        .map(|line| line.split_at(line.len() / 2))
        .map(|(compartment1, compartment2)| {
            let c1: BTreeSet<_> = compartment1.chars().collect();
            let c2: BTreeSet<_> = compartment2.chars().collect();

            c1.intersection(&c2).copied().collect::<Vec<char>>()
        })
        .map(|intersection| {
            intersection
                .iter()
                .map(|c| {
                    if c.is_lowercase() {
                        *c as usize - 96
                    } else {
                        *c as usize - 38
                    }
                })
                .collect::<BTreeSet<_>>()
        })
        .filter_map(|translated_intersect| translated_intersect.into_iter().next())
        .collect_vec()
        .iter()
        .sum::<usize>();

    println!("Sum of priorities: {}", sum_of_priorities);

    Ok(())
}
