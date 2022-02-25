public static long fib(int n, int m) {
    // [babies, adults]
    long[][] table = new long[n][2];
    table[0] = new long[]{1, 0};
    table[1] = new long[]{0, 1};
    for (int i = 2; i < n; i++) {
        long babies = table[i - 1][1];
        long adults = table[i - 1][0] + table[i - 1][1];
        if (i-m >= 0) {
            table[i] = new long[]{babies, adults - table[i - m][0]};
        } else {
            table[i] = new long[]{babies, adults};
        }
    }
    return table[n - 1][0] + table[n - 1][1];
}
