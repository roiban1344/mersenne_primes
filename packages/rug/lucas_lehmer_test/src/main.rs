use rug::ops::Pow;
use rug::Integer;

//Returns a list of prime numbers less than n.
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

fn mersenne_number(n: u32) -> Integer {
    (Integer::from(1) << n) - 1
}

//Executes the Lucas-Lehmer test for p-th Mersenne number.
//p must be an odd prime number.
fn lucas_lehmer_test(p: u32) -> bool {
    let m = mersenne_number(p);
    let mut s = Integer::from(4);
    for _ in 2..=p - 1 {
        s = s.pow(2) - 2;
        while s >= m {
            s = Integer::from(&s >> p) + (s & &m);
            if s == m {
                s = Integer::from(0);
                break;
            }
        }
    }
    s == 0
}

fn main() {
    let primes = primes(120000);
    for p in primes {
        print!("{},", p);
        if p != 2 {
            if lucas_lehmer_test(p) {
                println!();
                println!("{}", p);
            }
        }
    }
}
