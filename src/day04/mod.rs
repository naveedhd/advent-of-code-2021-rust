type Inputs = Vec<usize>;
type BoardValue = (usize, bool);
type Board = Vec<BoardValue>;
type Boards = Vec<Board>;


pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    let inputs = &data.0;
    let boards = &data.1;

    println!("{:?}", solve1(inputs, boards.clone().as_mut_slice()));
    println!("{:?}", solve2(&data.0, boards.clone().as_mut_slice()));
}

fn parse(file_string: &str) -> (Inputs, Boards) {
    let input_board_split = file_string.split_once('\n').unwrap();
    let input_str = input_board_split.0;
    let boards_str = input_board_split.1;

    let inputs = input_str.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    let boards =
        boards_str
            .split("\n\n")
            .map(|board_str| board_str
                                .split_whitespace()
                                .map(|s| (s.parse::<usize>().unwrap(), false))
                                .collect())
            .collect();


    (inputs, boards)
}

fn check_board(board: &[BoardValue], pos: usize) -> bool {
    let col = pos / 5;
    let row_start = col * 5;
    let mut row_iter = board.iter().skip(row_start).take(5);

    let row = pos % 5;
    let mut col_iter = board.iter().skip(row).step_by(5);

    row_iter.all(|x| x.1) || col_iter.all(|x| x.1)
}

fn mark_board(board: &mut [BoardValue], input: usize) -> bool {
    match board.iter().position(|board_val| !board_val.1 && board_val.0 == input) {
        Some(pos) => {
            board[pos].1 = true;
            check_board(board, pos)
        },
        None => false
    }
}

fn board_score(board: &Board) -> usize {
    board
        .iter()
        .filter_map(|board_val| if !board_val.1 {Some(board_val.0)} else { None })
        .sum()
}


fn solve1(inputs: &Inputs, boards: &mut [Board]) -> usize {
    for input in inputs {
        for board in boards.iter_mut() {
            if mark_board(board, *input) {
                return input * board_score(board)
            }
        }
    }

    0
}

fn solve2(inputs: &Inputs, boards: &mut [Board]) -> usize {
    let mut last_score = 0;

    let mut boards_done = vec![false; boards.len()];
    for input in inputs {
        for (i, board) in boards.iter_mut().enumerate() {
            if boards_done[i] {
                continue;
            }

            if mark_board(board, *input) {
                last_score = input * board_score(board);
                boards_done[i] = true;
            }
        }
    }

    last_score
}
