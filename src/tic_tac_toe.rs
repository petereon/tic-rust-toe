#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub mark: char,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub content: Vec<Option<Player>>,
    pub size: u8,
}

type Coords = (u8, u8);

pub fn create_board(size: u8) -> Board {
    return Board {
        content: vec![None; (size * size).into()],
        size: size,
    };
}

pub fn create_player(mark: char) -> Player {
    return Player { mark: mark };
}

pub fn place_on_board(board: Board, _player: Player, _position: Coords) -> Board {
    let mut new_board = board;
    let position: u8 = ((_position.0 - 1) * new_board.size + (_position.1 - 1)).into();
    _ = std::mem::replace(&mut new_board.content[usize::from(position)], Some(_player));
    return new_board;
}
