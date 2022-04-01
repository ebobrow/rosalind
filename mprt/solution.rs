/*
 * Dependencies in Cargo.toml:
 * reqwest = "0.11"
 * tokio = { version = "1", features = ["full"] }
 */
use std::{collections::HashMap, error::Error};

use reqwest::Client;

async fn mprt(ids: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut occurrences: HashMap<&str, Vec<_>> = HashMap::new();
    for id in ids {
        let body = client
            .get(format!("https://www.uniprot.org/uniprot/{}.fasta", id))
            .send()
            .await?
            .text()
            .await?;
        let (_title, contents) = body.split_once('\n').unwrap();
        let contents: String = contents.chars().filter(|c| c != &'\n').collect();
        for i in 0..contents.len() {
            if &contents[i..i + 1] == "N" {
                if &contents[i + 1..i + 2] != "P" {
                    if &contents[i + 2..i + 3] == "S" || &contents[i + 2..i + 3] == "T" {
                        if &contents[i + 3..i + 4] != "P" {
                            // occurrences.push((id, i));
                            let arr = occurrences.entry(id).or_default();
                            arr.push(i + 1);
                            // println!("{}", &contents[i..i + 4]);
                        }
                    }
                }
            }
        }
    }
    for (key, value) in occurrences {
        println!("{}", key);
        for v in value {
            print!("{} ", v);
        }
        println!();
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let input = r#"A1X8C2
P07925
P21809_PGS1_BOVIN
P00743_FA10_BOVIN
P10761_ZP3_MOUSE
B5FPF2
A5WBR3
B5FNU0
Q00001_RHGA_ASPAC
Q28409
Q5U1Y9
P11831_SRF_HUMAN"#;
    let _ = mprt(input.split('\n').collect()).await;
}
