use num_bigint::BigInt;
use num_traits::identities::One;

/**
 * $2^n - 2^{\frac{n+1}{2}} - 1$
 */
pub fn critical_value(n: u32) -> BigInt {
    (BigInt::one() << n) - (BigInt::one() << ((n + 1) / 2)) - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn succeeds_squaring_for_2110() {
        let a = critical_value(2110);
        a.pow(2);
    }

    #[test]
    fn succeeds_squaring_for_2111() {
        let a = critical_value(2111);
        a.pow(2);
    }

    #[test]
    #[should_panic]
    fn fails_squaring_for_2112() {
        let a = critical_value(2112);
        a.pow(2);
    }

    #[test]
    #[should_panic]
    fn fails_squaring_for_2113() {
        let a = critical_value(2113);
        a.pow(2);
    }


    #[test]
    #[should_panic]
    fn fails_squaring_for_31415() {
        let a = critical_value(31415);
        a.pow(2);
    }

    #[test]
    #[should_panic]
    fn fails_squaring_for_2112_with_prod() {
        let a = critical_value(2112);
        let _ = &a * &a;
    }

}
