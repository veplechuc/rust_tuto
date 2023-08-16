use anyhow::{Context, Result}; // add the anyhow library
use std::fs::File;
use Puzzle;

fn get_puzzle(filename: &str) -> Result<Puzzle> {
    // dont need to define the error type
    let fh = File::open(filename) //
        .with_context(|| format!("couldn't open the puzzle file {}", filename))?; // passing to error a context
    let puzzle = Puzzle::from_file(fh).context("couldn't convert data into a puzzle")?;
    Ok(puzzle)
}

fn main() -> Result<()> {
    let puzzle = get_puzzle("puzzle_file_path").context("Couldn't get the first puzzle")?;
    println!("playing ->", puzzle.name);
    Ok(())
}
