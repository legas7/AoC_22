use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    _ = File::open("inputs/day4_input.txt")?.read_to_string(&mut buffer)?;

    let overlaps_sum = buffer
        .lines()
        .flat_map(|line| line.split(',').collect_tuple::<(_, _)>())
        .map(|pair_sections| {
            (
                section_to_hashset(pair_sections.0),
                section_to_hashset(pair_sections.1),
            )
        })
        .map(|(e1, e2)| e1.is_superset(&e2) || e2.is_superset(&e1))
        .fold(0, |acc, contains| if contains { acc + 1 } else { acc });

    println!("Fully overlapping pairs : {}", overlaps_sum);

    Ok(())
}

fn section_to_hashset(s: &str) -> HashSet<u32> {
    s.split("-")
        .tuples::<(_, _)>()
        .map(|t| (t.0.parse::<u32>().unwrap(), t.1.parse::<u32>().unwrap()))
        .map(|pt| {
            (pt.0..pt.1 + 1)
                .collect_vec()
                .into_iter()
                .collect::<HashSet<_>>()
        })
        .collect_vec()
        .first()
        .cloned()
        .unwrap()
}
