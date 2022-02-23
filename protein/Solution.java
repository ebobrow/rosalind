import java.util.HashMap;

public static String protein(String rna) {
    HashMap<String, String> codons = new HashMap<>();
    codons.put("UUU", "F");
    codons.put("CUU", "L");
    codons.put("AUU", "I");
    codons.put("GUU", "V");
    codons.put("UUC", "F");
    codons.put("CUC", "L");
    codons.put("AUC", "I");
    codons.put("GUC", "V");
    codons.put("UUA", "L");
    codons.put("CUA", "L");
    codons.put("AUA", "I");
    codons.put("GUA", "V");
    codons.put("UUG", "L");
    codons.put("CUG", "L");
    codons.put("AUG", "M");
    codons.put("GUG", "V");
    codons.put("UCU", "S");
    codons.put("CCU", "P");
    codons.put("ACU", "T");
    codons.put("GCU", "A");
    codons.put("UCC", "S");
    codons.put("CCC", "P");
    codons.put("ACC", "T");
    codons.put("GCC", "A");
    codons.put("UCA", "S");
    codons.put("CCA", "P");
    codons.put("ACA", "T");
    codons.put("GCA", "A");
    codons.put("UCG", "S");
    codons.put("CCG", "P");
    codons.put("ACG", "T");
    codons.put("GCG", "A");
    codons.put("UAU", "Y");
    codons.put("CAU", "H");
    codons.put("AAU", "N");
    codons.put("GAU", "D");
    codons.put("UAC", "Y");
    codons.put("CAC", "H");
    codons.put("AAC", "N");
    codons.put("GAC", "D");
    codons.put("UAA", "Stop");
    codons.put("CAA", "Q");
    codons.put("AAA", "K");
    codons.put("GAA", "E");
    codons.put("UAG", "Stop");
    codons.put("CAG", "Q");
    codons.put("AAG", "K");
    codons.put("GAG", "E");
    codons.put("UGU", "C");
    codons.put("CGU", "R");
    codons.put("AGU", "S");
    codons.put("GGU", "G");
    codons.put("UGC", "C");
    codons.put("CGC", "R");
    codons.put("AGC", "S");
    codons.put("GGC", "G");
    codons.put("UGA", "Stop");
    codons.put("CGA", "R");
    codons.put("AGA", "R");
    codons.put("GGA", "G");
    codons.put("UGG", "W");
    codons.put("CGG", "R");
    codons.put("AGG", "R");
    codons.put("GGG", "G");

    StringBuffer protein = new StringBuffer();
    for (int i = 0; i < rna.length(); i += 3) {
        String prot = codons.get(rna.substring(i, i + 3));
        if (prot.equals("Stop")) {
            break;
        }
        protein.append(prot);
    }
    return String.valueOf(protein);
}
