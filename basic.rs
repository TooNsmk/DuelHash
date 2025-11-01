use dualhash::{dualhash1024, dualhash512_trunc};

#[test]
fn test_lengths() {
    assert_eq!(dualhash1024(b"abc").len(), 128);
    assert_eq!(dualhash512_trunc(b"abc").len(), 64);
}
