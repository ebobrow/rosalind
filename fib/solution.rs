fn fib(n: usize, k: usize) -> usize {
    let mut table = vec![1; n];
    for i in 2..n {
        table[i] = table[i - 1] + k * table[i - 2];
    }
    table[n - 1]
}
