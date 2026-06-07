use broken_app::{algo, concurrency, leak_buffer, normalize, sum_even, average_positive};

#[test]
fn sums_even_numbers() {
    assert_eq!(sum_even(&[1,2,3,4]), 6);
}

#[test]
fn counts_non_zero_bytes() {
    assert_eq!(leak_buffer(&[0,1,0,2,3]), 3);
}

#[test]
fn dedup_preserves_uniques() {
    assert_eq!(algo::slow_dedup(&[5,5,1,2,2,3]), vec![1,2,3,5]);
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
    assert_eq!(normalize("Hello\t\tWorld"), "helloworld");
}

#[test]
fn averages_only_positive() {
    assert!((average_positive(&[-5,5,15]) - 10.0).abs() < f64::EPSILON);
    assert_eq!(average_positive(&[]), 0.0);
    assert_eq!(average_positive(&[-1,-2]), 0.0);
}

#[test]
fn race_increment_is_correct() {
    assert_eq!(concurrency::race_increment(1000,4), 4000);
}