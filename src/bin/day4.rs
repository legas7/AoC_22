use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    _ = File::open("inputs/day4_input.txt")?.read_to_string(&mut buffer)?;

    let overlaps_sum = buffer
        .lines()
        .flat_map(|line| line.split(',').collect_tuple::<(_, _)>())
        .map(|(s1, s2)| (section_to_hashset(s1), section_to_hashset(s2)))
        .map(|(hs1, hs2)| hs1.is_disjoint(&hs2))
        .fold(
            0,
            |acc, is_disjoint| if is_disjoint { acc } else { acc + 1 },
        );

    println!("Partially overlapping pair sections : {}", overlaps_sum);

    Ok(())
}

fn section_to_hashset(section: &str) -> HashSet<u32> {
    section
        .split("-")
        .tuples::<(_, _)>()
        .map(|(beg_str, end_str)| {
            (
                beg_str.parse::<u32>().unwrap(),
                end_str.parse::<u32>().unwrap(),
            )
        })
        .map(|(beg_int, end_int)| {
            (beg_int..end_int + 1)
                .collect_vec()
                .into_iter()
                .collect::<HashSet<_>>()
        })
        .collect_vec()
        .first()
        .cloned()
        .unwrap()
}
