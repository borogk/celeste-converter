use celeste_converter::math::make_divisible_by;
use rstest::rstest;
use rstest_reuse::{apply, template};

#[template]
#[rstest]
#[case(0, 1, 0)]
#[case(0, 100, 0)]
#[case(1, 1, 1)]
#[case(1, 8, 8)]
#[case(1, 100, 100)]
#[case(8, 8, 8)]
#[case(8, 100, 100)]
#[case(100, 1, 100)]
#[case(100, 2, 100)]
#[case(100, 25, 100)]
#[case(100, 50, 100)]
#[case(100, 75, 150)]
#[case(990, 99, 990)]
#[case(991, 99, 1089)]
fn make_divisible_by_cases(#[case] value: usize, #[case] divider: usize, #[case] expected: usize) {}

#[apply(make_divisible_by_cases)]
fn make_divisible_by_has_correct_result(#[case] value: usize, #[case] divider: usize, #[case] expected: usize) {
    let actual = make_divisible_by(value, divider);
    assert_eq!(actual, expected, "{} made divisible by {} should be {}, but was {}", value, divider, expected, actual);
}
