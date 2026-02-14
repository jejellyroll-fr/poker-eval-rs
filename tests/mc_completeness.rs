#[cfg(test)]
mod tests {
    use poker_eval_rs::deck::StdDeck;
    use poker_eval_rs::enumdefs::{EnumResult, Game};
    use poker_eval_rs::enumerate::enum_sample;

    #[test]
    fn test_mc_completeness() {
        let (pocket1, _) = StdDeck::string_to_mask("As Ks").unwrap();
        let (pocket2, _) = StdDeck::string_to_mask("2s 2d").unwrap();
        let pockets = vec![pocket1, pocket2];
        let board = StdDeck::string_to_mask("").unwrap().0;
        let dead = StdDeck::string_to_mask("").unwrap().0;
        let mut result = EnumResult::new(Game::Holdem);

        // Stud
        result.clear();
        result.game = Game::Stud7;
        enum_sample(
            Game::Stud7,
            &pockets,
            board,
            dead,
            2,
            0,
            100,
            true,
            &mut result,
        )
        .expect("Stud7 MC failed");

        // Razz
        result.clear();
        result.game = Game::Razz;
        enum_sample(
            Game::Razz,
            &pockets,
            board,
            dead,
            2,
            0,
            100,
            true,
            &mut result,
        )
        .expect("Razz MC failed");

        // Stud8
        result.clear();
        result.game = Game::Stud78;
        enum_sample(
            Game::Stud78,
            &pockets,
            board,
            dead,
            2,
            0,
            100,
            true,
            &mut result,
        )
        .expect("Stud8 MC failed");

        // Lowball27 (pockets usually 5 cards, but 2 works too, just draws 3)
        result.clear();
        result.game = Game::Lowball27;
        enum_sample(
            Game::Lowball27,
            &pockets,
            board,
            dead,
            2,
            0,
            100,
            true,
            &mut result,
        )
        .expect("Lowball27 MC failed");
    }
}
