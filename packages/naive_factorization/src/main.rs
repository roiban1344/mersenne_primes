fn main() {
    let nums = [
        i8::MAX as i128,
        i16::MAX as i128,
        i32::MAX as i128,
        i64::MAX as i128,
        i128::MAX,
    ];
    for n in nums {
        for m in 2.. {
            let (q, r) = (n / m, n % m);
            if r == 0 {
                println!("{} is composite", n);
                break;
            }
            if q <= m {
                println!("{} is prime", n);
                break;
            }
        }
    }
}
