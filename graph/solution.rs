struct Dna {
    name: String,
    contents: String,
}

impl From<String> for Dna {
    fn from(s: String) -> Self {
        let (name, contents) = s.split_once("\n").unwrap();
        let name = name.to_string();
        let contents = contents.to_string().replace("\n", "");
        Self { name, contents }
    }
}

impl From<&str> for Dna {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

fn graph(input: String) {
    let dnas = input.split('>').skip(1).map(Dna::from).collect::<Vec<_>>();
    for dna in &dnas {
        let suffix = &dna.contents[dna.contents.len() - 3..];
        for dna2 in &dnas {
            if dna2.contents.starts_with(suffix) && dna2.contents != dna.contents {
                println!("{} {}", dna.name, dna2.name);
            }
        }
    }
}
