use broken_app::{algo, sum_even};
use std::time::Instant;

fn time_it(label: &str, f: impl FnOnce()) {
    let start = Instant::now();
    f();
    println!("{label}: {:?}", start.elapsed());
}

fn main() {
    let data: Vec<i64> = (0..50_000).collect();

    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    for _ in 0..3 {
        time_it("sum_even", || { let _ = sum_even(&data); });
        time_it("slow_fib", || { let _ = algo::slow_fib(32); });
        time_it("slow_dedup", || { let _ = algo::slow_dedup(&dedup_data); });
    }
}
