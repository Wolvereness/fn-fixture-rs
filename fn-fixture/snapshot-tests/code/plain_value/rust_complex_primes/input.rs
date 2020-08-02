{
    use std::collections::HashSet;
    const MAX: usize = 1000;
    const P1: usize = 2;
    let mut primes: HashSet<_> = (P1..MAX).collect();
    for p in P1..MAX {
        if !primes.contains(&p) {
            continue;
        }
        for f in p..MAX {
            let p = p * f;
            if p > MAX {
                break;
            }
            primes.remove(&p);
        }
    }
    let mut primes: Vec<_> = primes.into_iter().collect();
    primes.sort();
    primes
}