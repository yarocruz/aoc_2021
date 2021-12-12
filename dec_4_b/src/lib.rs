use ndarray::{
    arr2, aview_mut2, Array2, ArrayView, ArrayView2, ArrayViewMut2,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::opt,
    multi::{fill, separated_list1},
    IResult,
};
#[derive(Debug)]
pub struct BingoBoard<'a> {
    board: Array2<BingoBoardSquare<'a>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mark {
    Called,
    Uncalled,
}

#[derive(Debug, Clone, Copy)]
pub struct BingoBoardSquare<'a> {
    num: &'a str,
    called: Mark,
}

#[derive(Debug)]
pub enum BingoBoardState {
    Finished(u32),
    Playing,
}

impl<'a> BingoBoard<'a> {
    pub fn score(&self) -> u32 {
        self.board
            .iter()
            .filter_map(|BingoBoardSquare { num, called}| {
                match called {
                    Mark::Called => None,
                    Mark::Uncalled => {
                        Some(num.parse::<u32>().unwrap())
                    }
                }
            })
            .sum()
    }

    pub fn mark(&mut self, callout: &str) -> BingoBoardState {
        // Mark a cell, or square, basically the number on the board that's called
        for mut row in self.board.rows_mut() {
            for i in 0..row.len() {
                if row.get(i).unwrap().num == callout {
                    row.get_mut(i).unwrap().called = Mark::Called;
                }
            }
        }
        let row_winner =
            self.board.rows().into_iter().find(|row|{
                row.indexed_iter().all(|(_, item)| {
                    item.called == Mark::Called
                })
            });

        let col_winner =
            self.board.columns().into_iter().find(|col|{
                col.indexed_iter().all(|(_, item)| {
                    item.called == Mark::Called
                })
            });

        match (row_winner, col_winner) {
            // if theres no winner yet, keep the board playing
            (None, None) => BingoBoardState::Playing,
            // if a row or col, or both win then board wins and we take out with the score
            (None, Some(row))
            | (Some(row), None)
            | (Some(row), Some(_)) => BingoBoardState::Finished(
                self.score() * callout.parse::<u32>().unwrap(),
            )
        }
    }
}

// This is the part where we parse with Nom

// based on input file, return BingoBoardSquares with their values and initialize their called state with Mark::Uncalled
fn square(input: &str) -> IResult<&str, BingoBoardSquare> {
    let (input, value) = digit1(input)?;
    let (input, _) = opt(space1)(input)?;
    Ok(
        ( input, BingoBoardSquare {
        num: value,
        called: Mark::Uncalled
        })
    )
}
// here we create rows, with the square fn above
pub fn row(input: &str) -> IResult<&str, [BingoBoardSquare; 5]> {
    let (input, _) = opt(newline)(input)?;
    let (input, _) = opt(space1)(input)?;
    let mut buf = [BingoBoardSquare {
        num: "",
        called: Mark::Uncalled,
    }; 5];
    let (input, ()) = fill(square, &mut buf)(input)?;
    Ok((input, buf))
}

// here we create the board
pub fn board(input: &str) -> IResult<&str, BingoBoard> {
    let mut rows = [[BingoBoardSquare {
        num: "",
        called: Mark::Uncalled,
    }; 5]; 5];
    let (input, ()) = fill(row, &mut rows)(input)?;

    let arr = arr2(&rows);
    Ok((input, BingoBoard { board: arr}))
}

// this one is the equivalent of what we were doing when we .split("\n\n")
pub fn separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ()))
}

pub fn puzzle_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<BingoBoard>)> {
    let (input, callouts) = separated_list1(tag(","), digit1)(input)?;
    let (input, _) = separator(input)?;
    let (input, boards) = separated_list1(separator, board)(input)?;
    Ok((input, (callouts, boards)))
}

