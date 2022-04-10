fn pper(n: i32, k: i32) -> i32 {
    ((n - k + 1)..=n).fold(1, |acc, i| acc * i % 1000000)
}
