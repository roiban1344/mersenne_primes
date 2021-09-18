use num::bigint::BigInt;
use num::bigint::Sign;
use num::integer;
use num::Signed;

fn is_prime(n: u32) -> bool {
    if n == 0 {
        panic!();
    } else if n == 1 {
        return false;
    }
    for d in 2..=integer::sqrt(n) {
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn mersenne_number(n: u32) -> BigInt {
    (num::one::<BigInt>() << n) - num::one::<BigInt>()
}

fn lucas_lehmer_test(p: u32) -> bool {
    //p must be an odd prime
    let m = mersenne_number(p);
    let mut s = BigInt::new(Sign::Plus, vec![4]);
    for _ in 1..=p - 2 {
        s = &s * &s - BigInt::new(Sign::Plus, vec![2]);
        if s.is_negative() {
            s += &m;
        }
        while s >= m {
            let a = (&s >> p) << p;
            s = (&a >> p) + (a ^ s);
            if s == m {
                s = num::zero();
                break;
            }
        }
    }
    s == num::zero()
}

fn main() {
    for i in 3..1000 {
        if !is_prime(i) {
            continue;
        }
        if lucas_lehmer_test(i) {
            println!("{}", i)
        }
    }
}
