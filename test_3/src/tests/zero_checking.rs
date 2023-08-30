mod zero_checking {
    use crate::check_zeroes;

    #[test]
    fn correct_zeros_checking() {
        assert!(check_zeroes("00", 2));
        assert!(check_zeroes("abrakadabra0000", 4));
        assert!(check_zeroes(
            "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000",
            3
        ));
    }

    #[test]
    #[should_panic]
    fn incorrect_zeros_checking() {
        assert!(check_zeroes(
            "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000",
            4
        ));
    }
}
