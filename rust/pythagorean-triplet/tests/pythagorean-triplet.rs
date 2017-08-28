extern crate pythagorean_triplet;

#[test]
fn test_basic() {
    assert_eq!(pythagorean_triplet::find_for(3 + 4 + 5), Some(3 * 4 * 5));
}

#[test]
fn test_answer() {
    assert_eq!(pythagorean_triplet::find(), Some(31875000));
}
