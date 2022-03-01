fn gc(dnas: Vec<Dna>) {
    let mut highest = 0.0;
    let mut highest_name = String::new();
    for dna in dnas {
        let gc_content = dna
            .contents
            .chars()
            .filter(|c| c == &'G' || c == &'C')
            .collect::<String>()
            .len() as f32
            / dna.contents.len() as f32
            * 100.0;
        if gc_content > highest {
            highest = gc_content;
            highest_name = dna.name;
        }
    }
    println!("{}\n{}", highest_name, highest);
}
