import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Scanner;

public class Util {
    public static String readFile(String name) {
        StringBuffer contents = new StringBuffer();
        try {
            File file = new File("/Users/elliotbobrow/Downloads/rosalind_" + name + ".txt");
            Scanner scanner = new Scanner(file);
            while (scanner.hasNextLine()) {
                contents.append(scanner.nextLine()).append('\n');
            }
            scanner.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
        return contents.toString();
    }

    public static HashMap<String, String> readFASTAFileToHashMap(String name) {
        String contents = readFile(name);
        boolean skipFirst = true;
        HashMap<String, String> dnas = new HashMap<>();
        for (String item : contents.split(">")) {
            if (skipFirst) {
                skipFirst = false;
                continue;
            }
            StringBuffer dna = new StringBuffer();
            int firstReturn = item.indexOf('\n') + 1;
            String dnaWithReturns = item.substring(firstReturn, item.length() - 1);
            for (int i = 0; i < dnaWithReturns.length(); i++) {
                if (dnaWithReturns.charAt(i) != '\n') {
                    dna.append(dnaWithReturns.charAt(i));
                }
            }
            dnas.put(item.substring(0, firstReturn - 1), dna.toString());
        }
        return dnas;
    }

    public static ArrayList<String> readFASTAFileToArrayList(String name) {
        String contents = readFile(name);
        boolean skipFirst = true;
        ArrayList<String> dnas = new ArrayList<>();
        for (String item : contents.split(">")) {
            if (skipFirst) {
                skipFirst = false;
                continue;
            }
            StringBuffer dna = new StringBuffer();
            int firstReturn = item.indexOf('\n') + 1;
            String dnaWithReturns = item.substring(firstReturn, item.length() - 1);
            for (int i = 0; i < dnaWithReturns.length(); i++) {
                if (dnaWithReturns.charAt(i) != '\n') {
                    dna.append(dnaWithReturns.charAt(i));
                }
            }
            dnas.add(dna.toString());
        }
        return dnas;
    }
}
