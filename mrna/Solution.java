public static int mrna(String protein) {
    HashMap<Character, Integer> codons = new HashMap<>();
    codons.put('A', 4);
    codons.put('V', 4);
    codons.put('R', 6);
    codons.put('S', 6);
    codons.put('K', 2);
    codons.put('N', 2);
    codons.put('T', 4);
    codons.put('M', 1);
    codons.put('I', 3);
    codons.put('Q', 2);
    codons.put('H', 2);
    codons.put('P', 4);
    codons.put('L', 6);
    codons.put('W', 1);
    codons.put('.', 3); // Stop
    codons.put('C', 2);
    codons.put('Y', 2);
    codons.put('F', 2);
    codons.put('G', 4);
    codons.put('E', 2);
    codons.put('D', 2);
    int total = 1;
    for (int i = 0; i < protein.length(); i++) {
        if (protein.charAt(i) != '\n') {
            total = (total * codons.get(protein.charAt(i))) % 1000000;
        }
    }
    return (total * codons.get('.')) % 1000000;
}
