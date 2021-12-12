use dec_4_b::{board, BingoBoard, puzzle_input, BingoBoardState};
use ndarray::{Array1, Array2, Axis, Zip};
use std::fs;

/*
In part 2 we need to keep iterating through the boards until we find the last board to win

 */

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process(&file));
}

fn process(input: &str) -> u32 {
    let (_, (callouts, mut boards)) = puzzle_input(input).unwrap();

    let mut playing_boards = boards
        .iter_mut()
        .map(|board| (board, BingoBoardState::Playing))
        .collect::<Vec<(&mut BingoBoard, BingoBoardState)>>();

    let mut winning_boards:Vec<u32> = vec![];

    for call in callouts.iter() {
        for (board, state) in playing_boards
            .iter_mut()
            .filter(|(_, state)| match state {
                BingoBoardState::Finished(_) => false,
                BingoBoardState::Playing => true,
            })
        {
            *state = board.mark(call);

            match state {
                BingoBoardState::Finished(final_score) => {
                    winning_boards.push(*final_score);
                }
                BingoBoardState::Playing => {}
            }
        }
    }

    *winning_boards.iter().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &'static str = include_str!("./test-input.txt");

    #[test]
    fn test_demo_data() {
        assert_eq!(1924, process(input));
    }
}


//function to find winning board * we really don't need this
// fn find_winner(item: Vec<bool>) -> bool {
//     let lines = [
//         [0,1,2,3,4],
//         [5,6,7,8,9],
//         [10,11,12,13,14],
//         [15,16,17,18,19],
//         [20,21,22,23,24],
//         [0,5,10,15,20],
//         [1,6,11,16,21],
//         [2,7,12,17,22],
//         [3,8,13,18,23],
//         [4,9,14,19,24],
//
//     ];
//
//     let mut result:bool = false;
//
//     for i in 0..lines.len() {
//         let [a, b, c, d, e] = lines[i];
//         if item[a] == item[b] && item[a] == item[c] && item[a] == item[d] && item[a] == item[e] {
//             result = item[a];
//         }
//         // println!("{}", b);
//         // //println!("{:?}", item);
//     }
//
//     result
// }

