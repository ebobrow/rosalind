fn tran(a: String, b: String) -> f32 {
    let (transitions, transversions) = a.chars().zip(b.chars()).filter(|(a, b)| a != b).fold(
        (0, 0),
        |(transitions, transversions), (a, b)| {
            if (a == 'A' && b == 'G')
                || (a == 'G' && b == 'A')
                || (a == 'C' && b == 'T')
                || (a == 'T' && b == 'C')
            {
                (transitions + 1, transversions)
            } else {
                (transitions, transversions + 1)
            }
        },
    );
    transitions as f32 / transversions as f32
}
