use crate::calibrate::ICalibrateExt;
use assert2::assert;

#[test]
fn target_words_at_beginning_and_end() {
    // Given
    let sut = "onekadjgsknine";
    let expected_res = 19_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn target_words_overlap() {
    // Given
    let sut = "oneighte";
    let expected_res = 18_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn jumble_combo() {
    // Given
    let sut = "sds3onekasjfnine8sktwo";
    let expected_res = 32_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}
