use std::collections::BinaryHeap;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut file = File::open("inputs/day1_input.txt").await?;
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).await?;
    let raw_str = String::from_utf8(buffer)?;

    let mut sum_per_elf = raw_str
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split('\n')
                .map(|item| item.parse::<i32>().expect("Could not parse item"))
                .sum::<i32>()
        })
        .collect::<BinaryHeap<_>>();
    let best_equiped_elf_cal = sum_per_elf.pop().expect("Failed to find best equipped elf");
    let second_best_equiped_elf_cal = sum_per_elf
        .pop()
        .expect("Failed to find second best equipped elf");
    let third_best_equiped_elf_cal = sum_per_elf
        .pop()
        .expect("Failed to find third best equipped elf");

    println!("Best equipped elf has calories: {}", best_equiped_elf_cal);
    println!(
        "Top three elves have: {} calories combined",
        best_equiped_elf_cal + second_best_equiped_elf_cal + third_best_equiped_elf_cal
    );

    Ok(())
}
