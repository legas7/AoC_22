use itertools::Itertools;
use std::{fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    _ = File::open("inputs/day5_input.txt")?.read_to_string(&mut buffer)?;

    let raw_initial_state = buffer
        .lines()
        .take(8)
        .map(|l| {
            l.chars()
                .tuple_windows::<(_, _, _)>()
                .step_by(4)
                .map(|(_, s, _)| if s.is_alphabetic() { Some(s) } else { None })
                .collect_vec()
        })
        .collect_vec();

    let mut transposed_initial_state = vec![Vec::default(); 9];
    for row in raw_initial_state.into_iter().rev() {
        for (i, col) in transposed_initial_state.iter_mut().enumerate() {
            if let Some(Some(c)) = row.get(i).cloned() {
                col.push(c)
            }
        }
    }

    let moves = buffer
        .lines()
        .skip(10)
        .map(|line| line.split_whitespace())
        .map(|mut split_line| {
            split_line.next();
            split_line.step_by(2).collect_tuple::<(_, _, _)>().unwrap()
        })
        .map(|single_move| {
            (
                single_move.0.parse::<u32>().unwrap(),
                single_move.1.parse::<u32>().unwrap(),
                single_move.2.parse::<u32>().unwrap(),
            )
        })
        .collect_vec();

    let result = execute(transposed_initial_state, moves)?
        .iter()
        .map(|c| c.last().cloned().unwrap())
        .collect_vec();

    dbg!(result);

    Ok(())
}

fn execute(
    mut state: Vec<Vec<char>>,
    moves: Vec<(u32, u32, u32)>,
) -> anyhow::Result<Vec<Vec<char>>> {
    for (count, from, to) in moves.into_iter() {
        let source_stack_idx = (from - 1) as usize;
        let dest_stack_idx = (to - 1) as usize;

        let mut crane = Vec::new();
        for _ in 0u32..count {
            crane.push(state[source_stack_idx].pop());
        }

        state[dest_stack_idx].append(&mut crane.into_iter().rev().flatten().collect_vec());
    }

    Ok(state)
}
