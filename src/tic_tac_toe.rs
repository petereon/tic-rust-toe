#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Player {
    pub mark: char,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub content: Vec<Option<Player>>,
    pub size: u8,
}

type Coords = (u8, u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub players: Vec<Player>,
    pub end: bool,
    pub message: Option<String>,
}

pub fn initialize_game(players: Vec<Player>, board_size: u8) -> Game {
    return Game {
        board: create_board(board_size),
        current_player: players[0],
        players: players,
        end: false,
        message: None,
    };
}

pub fn create_board(size: u8) -> Board {
    return Board {
        content: vec![None; (size * size).into()],
        size: size,
    };
}

pub fn create_player(mark: char) -> Player {
    return Player { mark: mark };
}

pub fn index_from_coords(coords: Coords, board_size: u8) -> u8 {
    return (coords.0 - 1) * board_size + (coords.1 - 1);
}

pub fn place_on_board(board: Board, _player: Player, _position: Coords) -> Board {
    let mut new_board = board.clone();
    let position: u8 = index_from_coords(_position, board.size);
    new_board.content[usize::from(position)] = Some(_player);
    return new_board;
}

pub fn switch_players(players: Vec<Player>, current_player: Player) -> Player {
    let index =
        (players.iter().position(|&r| r == current_player).unwrap() + 1).rem_euclid(players.len());

    return players[index];
}

pub fn is_board_full(board: Board) -> bool {
    return board.content.iter().all(|&x| x.is_some());
}

pub fn check_victory(board: Board, player: Player) -> bool {
    // TODO: Test this
    if check_diagonal(&board, player) {
        return true;
    }
    if check_inverse_diagonal(&board, player) {
        return true;
    }
    for i in 1..board.size + 1 {
        let idx = index_from_coords((i, i), board.size);
        if board.content[idx as usize] == Some(player) {
            if check_horizontal(&board, i) {
                return true;
            }

            if check_vertical(&board, i) {
                return true;
            }
        }
    }
    return false;
}

pub fn check_vertical(board: &Board, i: u8) -> bool {
    for j in 1..board.size + 1 {
        let idx = index_from_coords((j, i), board.size);
        if board.content[idx as usize].is_none() {
            return false;
        }
    }
    return true;
}

pub fn check_horizontal(board: &Board, i: u8) -> bool {
    for j in 1..board.size + 1 {
        let idx = index_from_coords((i, j), board.size);
        if board.content[idx as usize].is_none() {
            return false;
        }
    }
    return true;
}

pub fn check_diagonal(board: &Board, player: Player) -> bool {
    for j in 1..board.size + 1 {
        let idx = index_from_coords((j, j), board.size);
        if board.content[idx as usize] != Some(player) {
            return false;
        }
    }
    return true;
}

pub fn check_inverse_diagonal(board: &Board, player: Player) -> bool {
    for j in 1..board.size + 1 {
        let idx = index_from_coords((j, board.size + 1 - j), board.size);
        if board.content[idx as usize] != Some(player) {
            return false;
        }
    }
    return true;
}
