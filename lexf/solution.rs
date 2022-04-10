// Cargo.toml:
// radix_fmt = "1.0"

use radix_fmt::radix;

fn lexf(alphabet: Vec<char>, n: usize) {
    for i in 0..(alphabet.len().pow(n.try_into().unwrap())) {
        let num = format!(
            "{:0width$}",
            u32::from_str_radix(
                &radix(i, alphabet.len().try_into().unwrap()).to_string(),
                10,
            )
            .unwrap(),
            width = n
        );
        let word: String = num
            .chars()
            .map(|c| {
                let idx: usize = c.to_digit(10).unwrap().try_into().unwrap();
                alphabet[idx]
            })
            .collect();
        println!("{}", word);
    }
}
