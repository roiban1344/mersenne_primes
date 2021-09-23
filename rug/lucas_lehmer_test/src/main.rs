use rug::ops::Pow;
use rug::Integer;

fn odd_primes(n: u32) -> Vec<u32> {
    let size = ((n - 1) >> 1) as usize;
    let mut is_prime = vec![true; size];
    let mut primes = vec![];
    for i in 0..size {
        if is_prime[i] {
            let p = 2 * i + 3;
            primes.push(p as u32);
            let mut j = 3 * i + 3;
            while j < size {
                is_prime[j] = false;
                j += p;
            }
        }
    }
    primes
}

fn mersenne_number(n: u32) -> Integer {
    (Integer::from(1) << n) - 1
}

fn lucas_lehmer_test(p: u32) -> bool {
    //p must be an odd prime
    let m = mersenne_number(p);
    let mut s = Integer::from(4);
    for _ in 1..=p - 2 {
        s = s.pow(2) - 2;
        if s < 0 {
            s += &m;
        }
        while s >= m {
            s = Integer::from(&s >> p) + Integer::from(s & &m);
            if s == m {
                s = Integer::from(0);
                break;
            }
        }
    }
    s == 0
}

fn main() {
    let odd_primes = odd_primes(30000);
    for p in odd_primes {
        if lucas_lehmer_test(p) {
            println!("{}", p);
        }
    }
}
