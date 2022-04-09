fn solution(k: usize) {
    println!("{}", factorial(k));
    perm(String::new(), (1..k + 1).collect());
}

fn factorial(k: usize) -> usize {
    if k == 0 {
        1
    } else {
        factorial(k - 1) * k
    }
}

fn perm(init: String, k: Vec<usize>) {
    if k.len() == 1 {
        println!("{} {}", init, k[0]);
        return;
    }
    for (i, el) in k.iter().enumerate() {
        let mut new = k.clone();
        new.remove(i);
        perm(format!("{} {}", init, el), new);
    }
}
