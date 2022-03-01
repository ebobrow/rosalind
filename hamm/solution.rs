fn hamm(a: String, b: String) -> usize {
    let mut diff = 0;
    for i in 0..a.len() {
        if a[i..i + 1] != b[i..i + 1] {
            diff += 1;
        }
    }
    diff
}

fn hamm_alt(a: String, b: String) -> usize {
    let mut diff = 0;
    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            diff += 1;
        }
    }
    diff
}
