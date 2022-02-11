public static String revc(String dna) {
    StringBuffer newDna = new StringBuffer(dna.length());
    for (int i = dna.length(); i > 0; i--) {
        switch (dna.charAt(i - 1)) {
            case 'T' -> newDna.append("A");
            case 'A' -> newDna.append("T");
            case 'G' -> newDna.append("C");
            case 'C' -> newDna.append("G");
        }
    }
    return newDna.toString();
}
