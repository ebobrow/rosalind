fn sseq(dnas: Vec<Dna>) {
    let sequence = &dnas[0].contents;
    let mut subsequence = dnas[1].contents.clone();
    for i in 0..sequence.len() {
        if sequence[i..i + 1] == subsequence[0..1] {
            print!("{} ", i + 1);
            subsequence.remove(0);
            if subsequence.len() == 0 {
                return;
            }
        }
    }
}
