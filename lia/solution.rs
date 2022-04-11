// Cargo.toml:
// num-bigint = "0.4"
use num_bigint::BigUint;

fn lia(k: u128, n: u128) -> f64 {
    let t = 2_u128.pow(k.try_into().unwrap());
    (n..=t)
        .map(|i| {
            ((1..=t).product::<BigUint>()
                / ((1..=i).product::<BigUint>() * (1..=(t - i)).product::<BigUint>()))
            .to_u64_digits()[0] as f64
                * 0.25_f64.powi(i.try_into().unwrap())
                * 0.75_f64.powi((t - i).try_into().unwrap())
        })
        .sum()
}
