mod find_hashes {
    use crate::find_hashes;

    #[test]
    fn correct_found_hashes() {
        let nums: Vec<u64> = find_hashes(3, 6).into_iter().map(|(x, _)| x).collect();

        assert_eq!(nums, vec![4163, 11848, 12843, 13467, 20215, 28892]);
    }

    #[test]
    #[should_panic]
    fn incorrect_found_hashes() {
        let nums: Vec<u64> = find_hashes(3, 6).into_iter().map(|(x, _)| x).collect();

        assert_eq!(nums, vec![4160, 11840, 12840, 13460, 20210, 28890]);
    }
}
