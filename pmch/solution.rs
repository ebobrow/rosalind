fn perfect_matchings(dna: String) -> i128 {
    let au = dna.chars().filter(|c| c == &'A').count();
    let gc = dna.chars().filter(|c| c == &'G').count();
    factorial(au.try_into().unwrap()) * factorital(gc.try_into().unwrap())
}

fn factorial(n: i128) -> i128 {
    if n == 0 {
        1
    } else {
        factorial(n - 1) * n
    }
}
