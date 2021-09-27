const PRIMES: [i32; 18] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
];

fn main() {
    for p in PRIMES {
        let m: u64 = (1 << p) - 1;
        {
            let mut m = m;
            let mut factors = vec![];
            for d in 2.. {
                while m % d == 0 {
                    m /= d;
                    factors.push(d);
                }
                if m <= d * d {
                    if m != 1 {
                        factors.push(m);
                    }
                    break;
                }
            }
            factors.sort();
            println!(
                "{:>2} {} {:?}",
                p,
                if factors.len() == 1 { "P" } else { "C" },
                factors
            );
        }
    }
}
