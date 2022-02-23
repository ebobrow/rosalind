public static void subs(String haystack, String needle) {
    int i = 0;
    while (true) {
        int index = haystack.indexOf(needle, i);
        if (index == -1) {
            break;
        }
        System.out.print(index + 1 + " ");
        i = index + 1;
    }
}
