use num_bigint::BigInt;
use num_bigint::Sign;
use num_traits::identities::{One, Zero};

fn sieve(n:u32, only_odd:bool)->Vec<u32>{
    let mut is_prime = vec![true;(n+1) as usize];
    let mut p = Vec::<u32>::new();
    for d in 2..=n{
        if is_prime[d as usize] {
            if d >=3 || !only_odd {
                p.push(d);
            }
            let mut k = 2 * d;
            while k <= n {
                is_prime[k as usize] = false;
                k += d;
            }
        }
    }
    p
}

fn mersenne_number(n: u32) -> BigInt {
    (BigInt::one() << n) - 1
}

fn lucas_lehmer_test(p: u32) -> bool {
    //p must be an odd prime
    let m = mersenne_number(p);
    let mut s = BigInt::new(Sign::Plus, vec![4]);
    for _ in 1..=p - 2 {
        s = s.pow(2) - 2;
        while s >= m {
            s = (&s >> p) + (s & &m);
            if s == m {
                s = BigInt::zero();
                break;
            }
        }
    }
    s.sign() == Sign::NoSign
}

fn main() {
    let p = sieve(1000, true);
    for i in p {
        if lucas_lehmer_test(i) {
            println!("{}", i)
        }
    }
}
