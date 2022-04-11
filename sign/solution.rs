fn solution(k: i32) {
    println!("{}", (1..=k).fold(1, |acc, x| acc * x * 2));
    perm(String::new(), (1..=k).collect());
}

fn perm(init: String, k: Vec<i32>) {
    if k.len() == 1 {
        println!("{} {}", init, k[0]);
        println!("{} {}", init, -k[0]);
        return;
    }
    for (i, el) in k.iter().enumerate() {
        let mut new = k.clone();
        new.remove(i);
        perm(format!("{} {}", init, el), new.clone());
        perm(format!("{} {}", init, -el), new);
    }
}
