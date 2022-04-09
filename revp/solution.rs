fn solution(dna: String) {
    for i in 0..dna.len() {
        for j in (i + 4)..(dna.len() + 1) {
            if revc(&dna[i..j]) == &dna[i..j] {
                println!("{} {}", i + 1, j - i);
            }
        }
    }
}

fn revc(dna: impl ToString) -> String {
    dna.to_string()
        .chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!(),
        })
        .collect()
}
