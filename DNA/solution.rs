fn dna(input: String) {
    println!(
        "{} {} {} {}",
        input.matches('A').count(),
        input.matches('C').count(),
        input.matches('G').count(),
        input.matches('T').count()
    );
}
