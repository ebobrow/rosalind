public static void graph(HashMap<String, String> dnas) {
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
