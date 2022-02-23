public static void gc(String in) {
    double max = 0;
    String maxName = "";
    boolean skipFirst = true;
    for (String item : in.split(">")) {
        if (skipFirst) {
            skipFirst = false;
            continue;
        }
        String dna = item.substring(item.indexOf("\n") + 1);
        double count = 0;
        int length = 0;
        for (int i = 0; i < dna.length(); i++) {
            if (dna.charAt(i) == 'C' || dna.charAt(i) == 'G') {
                count++;
            }
            if (dna.charAt(i) != '\n') {
                length++;
            }
        }
        double content = count/length * 100;
        if (content > max) {
            max = content;
            maxName = item.substring(0, item.indexOf("\n"));
        }
    }
    System.out.println(maxName);
    System.out.println(max);
}
