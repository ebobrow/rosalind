use std::cmp::max;

use util::Dna;

mod util;

fn cons(dnas: Vec<Dna>) {
    let mut a: Vec<usize> = Vec::new();
    let mut c: Vec<usize> = Vec::new();
    let mut g: Vec<usize> = Vec::new();
    let mut t: Vec<usize> = Vec::new();
    for i in 0..dnas.get(0).unwrap().contents.len() {
        let mut aa = 0;
        let mut cc = 0;
        let mut gg = 0;
        let mut tt = 0;
        for dna in &dnas {
            match &dna.contents[i..i + 1] {
                "A" => aa += 1,
                "C" => cc += 1,
                "G" => gg += 1,
                "T" => tt += 1,
                _ => panic!(),
            }
        }
        a.push(aa);
        c.push(cc);
        g.push(gg);
        t.push(tt);
    }
    let mut consensus = String::new();
    for i in 0..a.len() {
        let a_val = a[i];
        let c_val = c[i];
        let g_val = g[i];
        let t_val = t[i];
        let max = max(max(max(a_val, c_val), g_val), t_val);
        consensus.push(if max == a_val {
            'A'
        } else if max == c_val {
            'C'
        } else if max == g_val {
            'G'
        } else {
            'T'
        });
    }
    println!("{}", consensus);
    print!(
        "A: {}",
        a.iter().map(|el| format!("{} ", el)).collect::<String>()
    );
    println!();
    print!(
        "C: {}",
        c.iter().map(|el| format!("{} ", el)).collect::<String>()
    );
    println!();
    print!(
        "G: {}",
        g.iter().map(|el| format!("{} ", el)).collect::<String>()
    );
    println!();
    print!(
        "T: {}",
        t.iter().map(|el| format!("{} ", el)).collect::<String>()
    );
}
