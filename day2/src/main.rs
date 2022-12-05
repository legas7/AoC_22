use anyhow::{bail, Context};
use tokio::{fs::File, io::AsyncReadExt};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_figure_score(self) -> anyhow::Result<i32> {
        match self {
            Shape::Rock => Ok(1),
            Shape::Paper => Ok(2),
            Shape::Scissors => Ok(3),
        }
    }

    fn parse(c: char) -> anyhow::Result<Shape> {
        match c {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => bail!("Can't parse '{}' into Shape", c),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut file = File::open("inputs/day2_input.txt").await?;
    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).await?;
    let raw_str = String::from_utf8(buffer)?;

    let result_points = raw_str
        .split('\n')
        .map(get_result_points)
        .sum::<anyhow::Result<i32>>();

    println!("Got {} points", result_points?);

    Ok(())
}

fn get_result_points(line: &str) -> anyhow::Result<i32> {
    let opponents = Shape::parse(line.chars().nth(0).with_context(|| "empty input")?)?;
    let mine = Shape::parse(line.chars().nth(2).with_context(|| "empty input")?)?;

    let result_pts: anyhow::Result<i32> = match (&opponents, &mine) {
        (Shape::Rock, Shape::Rock) => Ok(3),
        (Shape::Rock, Shape::Paper) => Ok(6),
        (Shape::Rock, Shape::Scissors) => Ok(0),
        (Shape::Paper, Shape::Rock) => Ok(0),
        (Shape::Paper, Shape::Paper) => Ok(3),
        (Shape::Paper, Shape::Scissors) => Ok(6),
        (Shape::Scissors, Shape::Rock) => Ok(6),
        (Shape::Scissors, Shape::Paper) => Ok(0),
        (Shape::Scissors, Shape::Scissors) => Ok(3),
    };
    let figure_pts = mine.get_figure_score();

    Ok(result_pts? + figure_pts?)
}
