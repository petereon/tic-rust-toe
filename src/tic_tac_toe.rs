#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub mark: char,
}

pub fn create_board(size: u8) -> Vec<Option<Player>> {
    return vec![None; (size * size).into()];
}

pub fn create_player(mark: char) -> Player {
    return Player { mark: mark };
}
