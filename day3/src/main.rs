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
        .tuples::<(_, _, _)>()
        .map(|(pack1, pack2, pack3)| {
            let bts1: BTreeSet<_> = pack1.chars().collect();
            let bts2: BTreeSet<_> = pack2.chars().collect();
            let bts3: BTreeSet<_> = pack3.chars().collect();

            let bts1_bts2 = bts1.intersection(&bts2).copied().collect::<BTreeSet<_>>();
            bts1_bts2
                .intersection(&bts3)
                .copied()
                .collect::<BTreeSet<_>>()
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
