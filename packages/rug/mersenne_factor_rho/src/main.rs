use rug::ops::Pow;
use rug::{integer::IsPrime, Integer};
use std::collections::VecDeque;

const MILLER_RABIN_REPS: u32 = 64;

fn primes(n: u32) -> Vec<u32> {
    let mut is_prime = vec![true; n as usize];
    let mut primes = vec![];
    for i in 2..n {
        if is_prime[i as usize] {
            primes.push(i);
            for j in 2.. {
                let k = i * j;
                if !(k < n) {
                    break;
                }
                is_prime[k as usize] = false;
            }
        }
    }
    primes
}

fn main() {
    let primes = primes(260);
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
                    macro_rules! next {
                        ($x:expr) => {
                            ($x.pow(2) + c) % &n
                        };
                    }
                    x = next!(x);
                    y = next!(y);
                    y = next!(y);
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
