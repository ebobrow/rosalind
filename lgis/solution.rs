use std::{cmp::Ordering, fs};

fn longest_subsequence(arr: Vec<u32>, cmp: Ordering) {
    let mut table = vec![Vec::new(); arr.len()];
    table[0] = vec![arr[0]];
    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i].cmp(&arr[j]) == cmp && table[i].len() < table[j].len() + 1 {
                table[i] = table[j].clone();
                table[i].push(arr[i]);
            }
        }
        if table[i].len() == 0 {
            table[i] = vec![arr[i]];
        }
    }
    for el in table.into_iter().max_by_key(Vec::len).unwrap() {
        print!("{} ", el);
    }
    println!("\n");
}

fn main() {
    let arr: Vec<u32> = fs::read_to_string("/home/elliotbobrow/Downloads/rosalind_lgis.txt")
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
        .chars()
        .filter(|c| c != &'\n')
        .collect::<String>()
        .split(' ')
        .map(|n| u32::from_str_radix(n, 10).unwrap())
        .collect();
    longest_subsequence(arr.clone(), Ordering::Greater);
    longest_subsequence(arr, Ordering::Less);
}
