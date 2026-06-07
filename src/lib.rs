pub mod algo;
pub mod concurrency;

/// Сумма чётных чисел – безопасная итераторная версия.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|&v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов без утечек.
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = boxed.len();
    let raw = Box::into_raw(boxed); // *mut [u8]
    let ptr = raw as *mut u8;       // указатель на первый элемент
    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *ptr.add(i) != 0_u8 {
                count += 1;
            }
        }
        let _ = Box::from_raw(raw); // освобождаем память
    }
    count
}

/// Нормализация строки: удаляем все пробельные символы и приводим к нижнему регистру.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Среднее арифметическое только положительных чисел.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<i64> = values.iter().copied().filter(|&v| v > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    let sum: i64 = positives.iter().sum();
    sum as f64 / positives.len() as f64
}