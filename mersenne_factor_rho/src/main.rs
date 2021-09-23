use rug::ops::Pow;
use rug::{integer::IsPrime, Integer};
use std::collections::VecDeque;

const MILLER_RABIN_REPS: u32 = 64;

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

fn main() {
    let primes = odd_primes(128);
    for p in primes {
        let mut stack = VecDeque::<Integer>::new();
        let mut factors = Vec::<Integer>::new();
        let m: Integer = (Integer::from(1) << p) - 1;
        stack.push_back(m);
        while stack.len() > 0 {
            let mut n = stack.pop_front().unwrap();
            let mut c = 1;
            while n.is_probably_prime(MILLER_RABIN_REPS) == IsPrime::No {
                let mut x = Integer::from(2);
                let mut y = Integer::from(2);
                let mut d = Integer::from(1);
                while d == 1 {
                    macro_rules! rand_next {
                        ($x:expr) => {
                            ($x.pow(2) + c) % &n
                        };
                    }
                    x = rand_next!(x);
                    y = rand_next!(y);
                    y = rand_next!(y);
                    let diff = Integer::from(&x - &y).abs();
                    d = diff.gcd(&n);
                }
                if d == n {
                    c += 1;
                    continue;
                } else {
                    n /= &d;
                    if d.is_probably_prime(MILLER_RABIN_REPS) == IsPrime::No {
                        stack.push_back(d);
                    } else {
                        factors.push(d);
                    }
                }
            }
            factors.push(n);
        }
        factors.sort();
        let factorization = factors
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" * ");
        println!("M_{} = {}", p, factorization);
    }
}
