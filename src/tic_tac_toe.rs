#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    mark: char,
}

pub fn create_game(size: u8) -> Vec<Option<Player>> {
    return vec![None; (size * size).into()];
}
