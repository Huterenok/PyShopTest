mod hashing {
    use crate::hash_value;

    #[test]
    fn correct_hashing() {
        assert_eq!(
            hash_value(4163),
            "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"
        );
        assert_eq!(
            hash_value(11848),
            "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000"
        );
        assert_eq!(
            hash_value(12843),
            "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000"
        );
        assert_eq!(
            hash_value(13467),
            "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000"
        );
        assert_eq!(
            hash_value(2513638),
            "862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000"
        );
        assert_eq!(
            hash_value(3063274),
            "277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000"
        );
    }

    #[test]
    #[should_panic]
    fn incorrect_hashing() {
        assert_eq!(
            hash_value(4163),
            "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3001"
        );
    }
}
