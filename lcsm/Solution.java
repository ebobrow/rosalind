public static String longestCommonSubstring(ArrayList<String> dnas) {
    String longest = "";
    for (int i = 0; i < dnas.get(0).length(); i++) {
        for (int j = i + 2; j < dnas.get(0).length() + 1; j++) {
            if (j - i < longest.length()) {
                continue;
            }
            String substring = dnas.get(0).substring(i, j);
            boolean fits = true;
            for (String dna : dnas) {
                if (!dna.contains(substring)) {
                    fits = false;
                    break;
                }
            }
            if (fits) {
                longest = substring;
            }
        }
    }
    return longest;
}
