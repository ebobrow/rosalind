use std::cmp::{max, min};

use num_bigint::BigUint;

mod util;

fn maximum_matchings(dna: String) -> BigUint {
    let a = dna.chars().filter(|c| c == &'A').count();
    let u = dna.chars().filter(|c| c == &'U').count();
    let c = dna.chars().filter(|c| c == &'C').count();
    let g = dna.chars().filter(|c| c == &'G').count();
    perm(max(a, u), min(a, u)) * perm(max(c, g), min(c, g))
}

fn perm(n: usize, k: usize) -> BigUint {
    (1..=n).rev().take(k).product::<BigUint>()
}
