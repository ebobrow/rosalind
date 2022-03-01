fn graph(dnas: Vec<Dna>) {
    for dna in &dnas {
        let suffix = &dna.contents[dna.contents.len() - 3..];
        for dna2 in &dnas {
            if dna2.contents.starts_with(suffix) && dna2.contents != dna.contents {
                println!("{} {}", dna.name, dna2.name);
            }
        }
    }
}
