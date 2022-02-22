public static int hamm(String a, String b) {
    int hamm = 0;
    for (int i = 0; i < a.length(); i++) {
        if (a.charAt(i) != b.charAt(i)) {
            hamm++;
        }
    }
    return hamm;
}
