import java.util.ArrayList;

static void display(ArrayList<Integer> list) {
    for (int elem : list) {
        System.out.print(elem + " ");
    }
    System.out.println();
}

public static void cons(String in) {
    ArrayList<Integer> a = new ArrayList<>();
    ArrayList<Integer> c = new ArrayList<>();
    ArrayList<Integer> g = new ArrayList<>();
    ArrayList<Integer> t = new ArrayList<>();
    ArrayList<String> dnas = new ArrayList<>();
    boolean skipFirst = true;
    for (String item : in.split(">")) {
        if (skipFirst) {
            skipFirst = false;
            continue;
        }
        int firstReturn = item.indexOf('\n') + 1;
        int secondReturn = item.indexOf('\n', firstReturn);
        if (secondReturn < 0) {
            dnas.add(item.substring(firstReturn));
        } else {
            dnas.add(item.substring(firstReturn, secondReturn));
        }
    }
    for (int i = 0; i < dnas.get(0).length(); i++) {
        int as = 0;
        int cs = 0;
        int gs = 0;
        int ts = 0;
        for (String dna : dnas) {
            switch (dna.charAt(i)) {
                case 'A' -> as++;
                case 'C' -> cs++;
                case 'G' -> gs++;
                case 'T' -> ts++;
            }
        }
        a.add(as);
        c.add(cs);
        g.add(gs);
        t.add(ts);
    }
    for (int i = 0; i < a.size(); i++) {
        int max = Math.max(Math.max(Math.max(a.get(i), c.get(i)), g.get(i)), t.get(i));
        if (max == a.get(i)) {
            System.out.print('A');
        } else if (max == c.get(i)) {
            System.out.print('C');
        } else if (max == g.get(i)) {
            System.out.print('G');
        } else {
            System.out.print('T');
        }
    }
    System.out.println();
    System.out.print("A: ");
    display(a);
    System.out.print("C: ");
    display(c);
    System.out.print("G: ");
    display(g);
    System.out.print("T: ");
    display(t);
}
