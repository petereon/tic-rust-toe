#[cfg(test)]
mod tic_tac_toe_test {
    use tic_rust_toe::tic_tac_toe::{self, initialize_game, Board, Game, Player};

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

    #[test]
    fn test_initialize_game() {
        assert_eq!(
            initialize_game(vec![Player { mark: 'X' }, Player { mark: 'O' }], 2),
            Game {
                board: Board {
                    content: vec![None, None, None, None],
                    size: 2
                },
                current_player: Player { mark: 'X' },
                players: vec![Player { mark: 'X' }, Player { mark: 'O' }],
                end: false,
                message: None
            }
        )
    }

    #[test]
    fn test_switch_players() {
        assert_eq!(
            tic_tac_toe::switch_players(
                vec![Player { mark: 'X' }, Player { mark: 'O' }],
                Player { mark: 'X' }
            ),
            Player { mark: 'O' }
        );
        assert_eq!(
            tic_tac_toe::switch_players(
                vec![Player { mark: 'X' }, Player { mark: 'O' }],
                Player { mark: 'O' }
            ),
            Player { mark: 'X' }
        );
    }
}
