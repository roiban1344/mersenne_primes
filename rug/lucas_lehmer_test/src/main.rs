use rug::ops::Pow;
use rug::Integer;

fn odd_primes(n: u32) -> Vec<u32> {
    let size = ((n - 1) / 2) as usize;
    let mut is_prime = vec![true; size];
    let mut primes = Vec::<u32>::new();
    for k in 0..size {
        if is_prime[k] {
            let p = (2 * k + 3) as u32;
            primes.push(p);
            let mut m = 3 * p;
            while m <= n {
                is_prime[((m-3)/2) as usize] = false;
                m += 2 * p;
            }
        }
    }
    primes
}

fn mersenne_number(n: u32) -> Integer {
    let one = Integer::from(1);
    Integer::from(&one << n) - one
}

fn lucas_lehmer_test(p: u32) -> bool {
    //p must be an odd prime
    let m = mersenne_number(p);
    let two = Integer::from(2);
    let mut s = Integer::from(4);
    for _ in 1..=p - 2 {
        s = Integer::from(s.pow(2)) - &two;
        if s < 0 {
            s += &m;
        }
        while s >= m {
            s = Integer::from(&s >> p) + Integer::from(&s & &m);
            if s == m {
                s = Integer::from(0);
                break;
            }
        }
    }
    s.signum() == 0
}

fn main() {
    let odd_primes = odd_primes(30000);
    for p in odd_primes {
        if lucas_lehmer_test(p) {
            println!("{}", p);
        }
    }
}
