#[cfg(test)]
mod tic_tac_toe_test {
    use tic_rust_toe::tic_tac_toe::create_game;

    #[test]
    fn test_create_game() {
        assert_eq!(
            create_game(3),
            [None, None, None, None, None, None, None, None, None]
        )
    }
}
