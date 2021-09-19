use num::bigint::BigInt;
use num::bigint::Sign;
use num::{Signed, Zero};

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
    (num::one::<BigInt>() << n) - num::one::<BigInt>()
}

fn lucas_lehmer_test(p: u32) -> bool {
    //p must be an odd prime
    let m = mersenne_number(p);
    let two = BigInt::new(Sign::Plus, vec![2]);
    let mut s = BigInt::new(Sign::Plus, vec![4]);
    for _ in 1..=p - 2 {
        s = &s * &s - &two;
        if s.is_negative() {
            s += &m;
        }
        while s >= m {
            s = (&s >> p) + (s & &m);
            if s == m {
                s = num::zero();
                break;
            }
        }
        // if i < p-2 && s.is_zero() {
        //     println!("{} {}", p, i);
        // }
    }
    s.is_zero()
}

fn main() {
    let p = sieve(30000, true);
    for i in p {
        if lucas_lehmer_test(i) {
            println!("{}", i)
        }
    }
}
