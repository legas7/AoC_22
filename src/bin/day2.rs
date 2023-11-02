use anyhow::{bail, Context};
use aoc22::NEWLINE;
use tokio::{fs::File, io::AsyncReadExt};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum ExpectedGameResult {
    Loss,
    Draw,
    Win,
}

impl ExpectedGameResult {
    fn parse(c: char) -> anyhow::Result<ExpectedGameResult> {
        match c {
            'X' => Ok(ExpectedGameResult::Loss),
            'Y' => Ok(ExpectedGameResult::Draw),
            'Z' => Ok(ExpectedGameResult::Win),
            _ => bail!("Can't parse '{}' into ExpectedGameResult", c),
        }
    }

    fn get_expected_shape(self, opponents_shape: &Shape) -> Shape {
        match (self, opponents_shape) {
            (ExpectedGameResult::Loss, Shape::Rock) => Shape::Scissors,
            (ExpectedGameResult::Loss, Shape::Paper) => Shape::Rock,
            (ExpectedGameResult::Loss, Shape::Scissors) => Shape::Paper,
            (ExpectedGameResult::Draw, Shape::Rock) => Shape::Rock,
            (ExpectedGameResult::Draw, Shape::Paper) => Shape::Paper,
            (ExpectedGameResult::Draw, Shape::Scissors) => Shape::Scissors,
            (ExpectedGameResult::Win, Shape::Rock) => Shape::Paper,
            (ExpectedGameResult::Win, Shape::Paper) => Shape::Scissors,
            (ExpectedGameResult::Win, Shape::Scissors) => Shape::Rock,
        }
    }
}

impl Shape {
    fn get_figure_score(self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn parse(c: char) -> anyhow::Result<Shape> {
        match c {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
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
        .split(NEWLINE)
        .map(get_result_points)
        .sum::<anyhow::Result<i32>>();

    println!("Got {} points", result_points?);

    Ok(())
}

fn get_result_points(line: &str) -> anyhow::Result<i32> {
    let opponents_shape = Shape::parse(line.chars().nth(0).with_context(|| "empty input")?)?;
    let expected_result =
        ExpectedGameResult::parse(line.chars().nth(2).with_context(|| "empty input")?)?;
    let my_expected_shape = expected_result.get_expected_shape(&opponents_shape);

    let result_pts = match (&opponents_shape, &my_expected_shape) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Rock, Shape::Scissors) => 0,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Scissors, Shape::Scissors) => 3,
    };
    let figure_pts = my_expected_shape.get_figure_score();

    Ok(result_pts + figure_pts)
}
