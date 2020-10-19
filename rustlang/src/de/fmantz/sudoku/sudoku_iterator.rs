use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use crate::sudoku_puzzle::SudokuPuzzleData;
use crate::sudoku_puzzle::SudokuPuzzle;
use crate::sudoku_io::NEW_SUDOKU_SEPARATOR;
use crate::sudoku_io::EMPTY_CHAR;
use crate::sudoku_io::QQWING_EMPTY_CHAR;
use crate::sudoku_puzzle::PUZZLE_SIZE;
use std::borrow::Borrow;
use std::str::Chars;

pub struct PuzzleLines {
    lines: io::Lines<io::BufReader<File>>
}

impl Iterator for PuzzleLines {

    type Item = SudokuPuzzleData;

    fn next(&mut self) -> Option<SudokuPuzzleData> {
        //Find first line wiht data:
        let mut first_line = self.re_init();
        if first_line.is_none() {
            return None
        }

        //Allocate memory for new puzzle:
        let mut puzzle: SudokuPuzzleData = SudokuPuzzleData::new();

        //Read first line:
        let mut line_data : String  = first_line.unwrap().unwrap();
        PuzzleLines::read_line(& mut line_data, &mut puzzle, 0);

        //Read other lines:
        for row in 1.. (PUZZLE_SIZE - 1) {
            let next_line = self.lines.next();
            if next_line.is_none() {
                return None;
            }
            let next_line_data = next_line.unwrap();
            if next_line_data.is_err() {
                return None;
            }
            PuzzleLines::read_line(&mut next_line_data.unwrap(), &mut puzzle, row);
        }
        return Some(puzzle);
    }
}

impl PuzzleLines {

    pub fn new(lines: io::Lines<io::BufReader<File>>) -> Self {
        PuzzleLines {
            lines: lines
        }
    }

    fn re_init(&mut self) -> Option<Result<String, Error>> {
        let mut cur_line = self.lines.next();
        while (cur_line.is_some()) {
            match cur_line.get_or_insert(Ok("".to_string())) {
                Err(_) => {
                    cur_line = None;
                    break;
                },
                Ok(cur_line_string) => {
                    if cur_line_string.is_empty() || cur_line_string.starts_with(NEW_SUDOKU_SEPARATOR) {
                        cur_line = self.lines.next();
                    } else {
                        break;
                    }
                }
            };
        }
        cur_line
    }

    fn read_line(line_data: &mut String, puzzle: &mut SudokuPuzzleData, row: usize) {
        //Read string into puzzle
        let mut chars_of_line: Chars = line_data.chars();
        for col in 0..PUZZLE_SIZE {
            let ch = chars_of_line.next();
            if ch.is_some() {
                let char_unwrapped = ch.unwrap();
                let number: u8 = if char_unwrapped == QQWING_EMPTY_CHAR {
                    0
                } else {
                    (char_unwrapped as i32 - EMPTY_CHAR as i32) as u8 //result is in [0 - 9]
                };
                puzzle.set(row, col, number);
            }
        }
    }
}