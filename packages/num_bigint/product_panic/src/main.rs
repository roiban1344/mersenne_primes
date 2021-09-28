use num_bigint::BigInt;
use num_traits::identities::One;

const CRITICAL_VALUE: u32 = 2112;

fn main() {
    let a = (BigInt::one() << CRITICAL_VALUE) - 1;
    let _ = &a * &a;
}
