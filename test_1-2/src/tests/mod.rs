#[cfg(test)]
mod tests {
    use crate::{generate_game, get_score, Score, Stamp, OFFSET_MAX_STEP, TIMESTAMPS_COUNT};

    #[inline]
    fn setup_test() -> Vec<Stamp> {
        generate_game()
    }

    #[test]
    fn zero_score_first_stamp() {
        let game_stamps = setup_test();

        let expected = Score { away: 0, home: 0 }.into();

        let res = get_score(&game_stamps, 0);

        assert_eq!(res, expected);
    }

    #[test]
    fn possible_offset() {
        let game_stamps = generate_game();
        let offset = game_stamps[TIMESTAMPS_COUNT].offset;

        let expected = game_stamps[TIMESTAMPS_COUNT].score.into();

        let res = get_score(&game_stamps, offset);

        assert_eq!(res, expected);
    }

    #[test]
    fn take_last_score() {
        let game_stamps = generate_game();
        let stamp1 = game_stamps[1];
        let stamp2 = game_stamps[2];
        let offset = stamp2.offset - 1;

        let expected = stamp1.score.into();

        let res = get_score(&game_stamps, offset);

        assert_eq!(res, expected);
    }

    #[test]
    fn impossible_offset() {
        let game_stamps = setup_test();
        let offset = OFFSET_MAX_STEP * TIMESTAMPS_COUNT as i32 + 1;

        let expected = game_stamps.last().unwrap().score.into();

        let res = get_score(&game_stamps, offset);

        assert_eq!(res, expected);
    }
}
