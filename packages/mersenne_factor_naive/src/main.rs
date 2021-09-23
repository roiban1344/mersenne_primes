use rug::{integer::IsPrime, Integer};

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
    let primes = odd_primes(127);
    for p in primes {
        let mut m = (1 << p) - 1;
        if Integer::from(m).is_probably_prime(64) != IsPrime::No {
            println!("{} prime", p);
        }
        let mut factors = vec![];
        let mut d = 2 * (p as u128) + 1;
        while m > 1 && d * d <= m {
            if m % d == 0 {
                loop {
                    m /= d;
                    factors.push(d);
                    if m % d != 0 {
                        break;
                    }
                }
                if Integer::from(m).is_probably_prime(64) != IsPrime::No {
                    break;
                }
            }

            d += 2 * (p as u128);
        }
        if m > 1 {
            factors.push(m);
        }
        println!("{} {:?}", p, factors);
    }
}
