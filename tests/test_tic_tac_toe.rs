#[cfg(test)]
mod tic_tac_toe_test {
    use tic_rust_toe::tic_tac_toe::{self, Player};

    #[test]
    fn test_create_board() {
        assert_eq!(
            tic_tac_toe::create_board(3),
            [None, None, None, None, None, None, None, None, None]
        )
    }

    #[test]
    fn test_create_player() {
        assert_eq!(tic_tac_toe::create_player('X'), Player { mark: 'X' });
    }

    #[test]
    fn test_place_on_board() {
        assert_eq!(
            tic_tac_toe::place_on_board(
                [None, None, None, None, None, None, None, None, None].to_vec(),
                Player { mark: 'X' },
                (1, 1)
            ),
            [
                Some(Player { mark: 'X' }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]
        )
    }
}
