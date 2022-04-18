use num_bigint::BigUint;

#[derive(Debug)]
pub struct Dna {
    pub name: String,
    pub contents: String,
}

impl<T> From<T> for Dna
where
    T: ToString,
{
    fn from(s: T) -> Self {
        let s = s.to_string();
        let (name, contents) = s.split_once('\n').unwrap();
        let name = name.to_string();
        let contents = contents.to_string().replace('\n', "");
        Self { name, contents }
    }
}

impl Dna {
    pub fn vec(input: impl ToString) -> Vec<Self> {
        input
            .to_string()
            .split('>')
            .skip(1)
            .map(Dna::from)
            .collect()
    }
}

pub fn choose(n: usize, k: usize) -> BigUint {
    (1..=n).product::<BigUint>()
        / ((1..=k).product::<BigUint>() * (1..=(n - k)).product::<BigUint>())
}
