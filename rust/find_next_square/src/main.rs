fn find_next_square(sq: u64) -> Option<u64> {
    if (sq as f64).sqrt().fract() != 0.0 {
        return None;
    } else {
        (sq + 1..).find(|x| {
            let sqrt = (*x as f64).sqrt();
            sqrt.fract() == 0.0
        })
    }
}

#[test]
fn sample_tests() {
    assert_eq!(find_next_square(121), Some(144));
    assert_eq!(find_next_square(625), Some(676));
    assert_eq!(find_next_square(319_225), Some(320_356));
    assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
    assert_eq!(find_next_square(155), None);
    assert_eq!(find_next_square(342_786_627), None);
}
