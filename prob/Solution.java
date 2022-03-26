public static double solution(String target, double gc) {
    double probability = 0;
    for (int i = 0; i < target.length(); i++) {
        switch (target.charAt(i)) {
            case 'C', 'G' -> probability += Math.log10(gc / 2);
            case 'A', 'T' -> probability += Math.log10((1.0 - gc) / 2);
        }
    }
    return probability;
}

public static void parseAndCall(String fileName) {
    String contents = Util.readFile(fileName);
    int firstLineBreak = contents.indexOf('\n');
    String seq = contents.substring(0, firstLineBreak);
    String[] gcs = contents.substring(firstLineBreak).split(" ");
    for (String gc : gcs) {
        System.out.print(solution(seq, Double.parseDouble(gc)));
        System.out.print(" ");
    }
}
