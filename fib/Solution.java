public static long fib(int n, int k) {
    long[] table = new long[n];
    table[0] = table[1] = 1;
    for (int i = 2; i < n; i++) {
        table[i] = table[i-1] + k*table[i-2];
    }
    return table[n - 1];
}
