public static double mendel(int k, int m, int n) {
    double t = k + m + n;
    double kk = k/t*(k-1)/(t-1);
    double km = 2*(k/t*m/(t-1));
    double kn = 2*(k/t*n/(t-1));
    double mm = m/t*(m-1)/(t-1);
    double mn = 2*(m/t*n/(t-1));
    return kk + km + kn + 0.75*mm + 0.5*mn;
}
