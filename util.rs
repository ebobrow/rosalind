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
        let (name, contents) = s.split_once("\n").unwrap();
        let name = name.to_string();
        let contents = contents.to_string().replace("\n", "");
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
