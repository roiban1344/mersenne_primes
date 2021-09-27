use num_bigint::BigInt;
use num_traits::identities::One;

fn repunit(n: u32)->BigInt{
    (BigInt::one() << n) - 1
}

fn main() {
    for i in 2..=10000 {
        println!("{}", i);
        let a = repunit(i);
        let _ = &a * &a;
    }
}
