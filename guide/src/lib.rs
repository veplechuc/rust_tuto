#![allow(dead_code)]
use std::fs::File;
use thiserror::Error;

pub const PUZZLE_PIECES: u32 = 42;

/// This is a Puzzle!
#[derive(Clone, Debug)]
pub struct Puzzle {
    /// Number of pieces
    pub num_pieces: u32,
    /// Descriptive name
    pub name: String,
}

impl Puzzle {
    /// Make a new puzzle!
    pub fn new() -> Self {
        let puzzle = Default::default();
        // info!("Created a puzzle with new(): {:?}", puzzle);
        puzzle
    }
    /// Load a puzzle from a file
    pub fn from_file(_fh: File) -> Result<Self, PuzzleError> {
        println!("HERE");
        // error!("This file is missing a piece!");
        Err(PuzzleError::MissingPiece)
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num_pieces: PUZZLE_PIECES,
            name: "Forest Lake".to_string(),
        }
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
    #[error("Missing a piece")]
    MissingPiece,
}

// must refer to a file
pub mod file_mod1;
// this case refers to a folder an inside a mod.rs
pub mod maths;

pub mod my_mod;

#[path = "bin/another_mod/math2.rs"]
pub mod math2;

//pub mod some_mode;
