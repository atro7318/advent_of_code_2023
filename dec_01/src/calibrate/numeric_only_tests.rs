use crate::calibrate::ICalibrateExt;
use assert2::assert;

#[test]
fn nums_at_beginning_and_end_of_string() {
    // Given
    let sut = "1abc2";
    let expected_res = 12;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn nums_in_middle_of_string() {
    // Given
    let sut = "pqr3stu8vwx";
    let expected_res = 38_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn lots_of_nums_in_string() {
    // Given
    let sut = "a1b2c3d4e5f";
    let expected_res = 15_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn one_num_in_string() {
    // Given
    let sut = "treb7uchet";
    let expected_res = 77_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn string_is_single_num() {
    // Given
    let sut = "3";
    let expected_res = 33_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}

#[test]
fn no_nums_found_is_zero() {
    // Given
    let sut = "sajdhfkdjshfk";
    let expected_res = 0_usize;

    // When
    let res = sut.calibrate();

    // Then
    assert!(res.unwrap() == expected_res)
}
