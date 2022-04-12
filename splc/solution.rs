mod util;

use itertools::Itertools;
use phf::phf_map;
use util::Dna;

static CODONS: phf::Map<&'static str, char> = phf_map! {
    "TTT" => 'F',
    "CTT" => 'L',
    "ATT" => 'I',
    "GTT" => 'V',
    "TTC" => 'F',
    "CTC" => 'L',
    "ATC" => 'I',
    "GTC" => 'V',
    "TTA" => 'L',
    "CTA" => 'L',
    "ATA" => 'I',
    "GTA" => 'V',
    "TTG" => 'L',
    "CTG" => 'L',
    "ATG" => 'M',
    "GTG" => 'V',
    "TCT" => 'S',
    "CCT" => 'P',
    "ACT" => 'T',
    "GCT" => 'A',
    "TCC" => 'S',
    "CCC" => 'P',
    "ACC" => 'T',
    "GCC" => 'A',
    "TCA" => 'S',
    "CCA" => 'P',
    "ACA" => 'T',
    "GCA" => 'A',
    "TCG" => 'S',
    "CCG" => 'P',
    "ACG" => 'T',
    "GCG" => 'A',
    "TAT" => 'Y',
    "CAT" => 'H',
    "AAT" => 'N',
    "GAT" => 'D',
    "TAC" => 'Y',
    "CAC" => 'H',
    "AAC" => 'N',
    "GAC" => 'D',
    "TAA" => '.',
    "CAA" => 'Q',
    "AAA" => 'K',
    "GAA" => 'E',
    "TAG" => '.',
    "CAG" => 'Q',
    "AAG" => 'K',
    "GAG" => 'E',
    "TGT" => 'C',
    "CGT" => 'R',
    "AGT" => 'S',
    "GGT" => 'G',
    "TGC" => 'C',
    "CGC" => 'R',
    "AGC" => 'S',
    "GGC" => 'G',
    "TGA" => '.',
    "CGA" => 'R',
    "AGA" => 'R',
    "GGA" => 'G',
    "TGG" => 'W',
    "CGG" => 'R',
    "AGG" => 'R',
    "GGG" => 'G',
};

fn dna_to_prot(dna: impl ToString) -> String {
    dna.to_string()
        .chars()
        .chunks(3)
        .into_iter()
        .map(|codon| CODONS.get(&codon.collect::<String>()[..]).unwrap())
        .collect()
}

fn splc(dnas: Vec<Dna>) -> String {
    dna_to_prot(
        dnas.into_iter()
            .map(|dna| dna.contents)
            .reduce(|acc, dna| acc.replace(&dna, ""))
            .unwrap(),
    )
}
