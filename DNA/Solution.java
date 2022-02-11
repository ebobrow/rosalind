public static void count(String dna) {
    int a = 0;
    int c = 0;
    int g = 0;
    int t = 0;
    for (int i = 0; i < dna.length(); i++) {
      switch (dna.charAt(i)) {
        case 'A' -> a++;
        case 'C' -> c++;
        case 'G' -> g++;
        case 'T' -> t++;
      }
    }
  System.out.println(a + " " + c + " " + g + " " + t);
}
