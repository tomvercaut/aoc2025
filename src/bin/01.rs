use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"; // TODO: Add the test input

fn parse<R: BufRead>(rdr: R) -> anyhow::Result<(i32, i32)> {
    let mut dail = 50;
    let mut part1 = 0;
    let mut part2 = 0;

    for line in rdr.lines() {
        let line = line?;
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let ts = chars.as_str();
        let value = ts.parse::<i32>()?;
        match dir {
            'L' => {
                let total = dail - value;
                dail = total % 100;
            }
            'R' => {
                let total = dail + value;
                dail = total % 100;
            }
            _ => {
                bail!("Direction is expected to be 'L' or 'R'.");
            }
        }
        part1 += i32::from(dail == 0);
    }

    Ok((part1, part2))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<(i32, i32)> {
        // TODO: Solve Part 1 of the puzzle
        // let answer = reader.lines().flatten().count();
        let answer = parse(reader)?;
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    let output = part1(BufReader::new(TEST.as_bytes()))?;
    assert_eq!(3, output.0);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let output = part1(input_file)?;
    let result = time_snippet!(output);
    println!("Result = {} | {}", result.0, result.1);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
