public static void graph(String in) {
    boolean skipFirst = true;
    HashMap<String, String> dnas = new HashMap<>();
    for (String item : in.split(">")) {
        if (skipFirst) {
            skipFirst = false;
            continue;
        }
        String dna;
        int firstReturn = item.indexOf('\n') + 1;
        dna = item.substring(firstReturn, item.length() - 1);
        String name = item.substring(0, item.indexOf("\n"));
        dnas.put(name, dna);
    }
    for (Map.Entry<String, String> entry : dnas.entrySet()) {
        String name = entry.getKey();
        String dna = entry.getValue();
        String suffix = dna.substring(dna.length() - 3);

        for (Map.Entry<String, String> entry1 : dnas.entrySet()) {
            if (entry1.getValue().substring(0, 3).equals(suffix) && !entry1.getValue().equals(dna)) {
                System.out.println(name + " " + entry1.getKey());
            }
        }
    }
}
