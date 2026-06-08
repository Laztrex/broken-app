use broken_app::{algo, leak_buffer, normalize, sum_even, average_positive, concurrency};

fn main() {
    let nums = [1, 2, 3, 4];
    println!("sum_even: {}", sum_even(&nums));

    let data = [1_u8, 0, 2, 3];
    println!("non-zero bytes: {}", leak_buffer(&data));

    let text = " Hello\tWorld ";
    println!("normalize: {}", normalize(text));

    let fib = algo::slow_fib(20);
    println!("fib(20): {}", fib);

    let uniq = algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]);
    println!("dedup: {:?}", uniq);

    let avg = average_positive(&[-5, 5, 15]);
    println!("average positive: {}", avg);

    let total = concurrency::race_increment(1000, 4);
    println!("race_increment: {}", total);
}