use itertools::Itertools;
use std::{fs::File, io::Read};
use tuple_conv::RepeatedTuple;

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    _ = File::open("inputs/day6_input.txt")?.read_to_string(&mut buffer)?;

    let marker_offset = buffer
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .flat_map(|(i, t)| {
            if t.to_vec().iter().all_unique() {
                Some(i + 4)
            } else {
                None
            }
        })
        .collect_vec()
        .first()
        .cloned();

    dbg!(marker_offset);

    Ok(())
}
