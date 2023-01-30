#[cfg(test)]
mod tic_tac_toe_test {
    use tic_rust_toe::tic_tac_toe::{self, Board, Player};

    #[test]
    fn test_create_board() {
        assert_eq!(
            tic_tac_toe::create_board(3).content,
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
                Board {
                    content: [None, None, None, None, None, None, None, None, None].to_vec(),
                    size: 3
                },
                Player { mark: 'X' },
                (1, 1)
            )
            .content,
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
        );
        assert_eq!(
            tic_tac_toe::place_on_board(
                Board {
                    content: [None, None, None, None, None, None, None, None, None].to_vec(),
                    size: 3
                },
                Player { mark: 'O' },
                (2, 2)
            )
            .content,
            [
                None,
                None,
                None,
                None,
                Some(Player { mark: 'O' }),
                None,
                None,
                None,
                None
            ]
        );
    }
}
