use rug::ops::Pow;
use rug::{integer::IsPrime, Integer};

// fn divmod_by_mersenne(&n: &Integer, k: u32) -> Integer {
//     let mut n = n;
//     let m = (Integer::from(2) << k) - 1;
//     while n >= m {
//         n = Integer::from(&n >> k) + Integer::from(n & &m);
//         if n == m {
//             n = Integer::from(0);
//             break;
//         }
//     }
//     n
// }

fn gen_pseudo_rand(x: Integer, modulo: &Integer) -> Integer {
    x % modulo
}

fn main() {
    let k = 101;
    let mut m: Integer = (Integer::from(1) << k) - 1;
    println!("M_{} {}", k, m);
    while m.is_probably_prime(64) == IsPrime::No {
        let mut x = Integer::from(2);
        let mut y = Integer::from(2);
        let mut d = Integer::from(1);
        while d == 1 {
            x = gen_pseudo_rand(x.pow(2) + 1, &m);
            y = gen_pseudo_rand(y.pow(2) + 1, &m);
            y = gen_pseudo_rand(y.pow(2) + 1, &m);
            let diff = Integer::from(&x - &y).abs();
            d = diff.gcd(&m);
        }
        if d == m {
            continue;
        } else {
            println!("{}", d);
            m /= d;
        }
    }
    println!("{}", m);
}
