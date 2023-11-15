use super::tests;

use num_bigint::BigUint;

/// This function generates the first prime with informed digits. It uses miller-rabin test of
/// composites.
pub fn generate_first_prime_from(digits: &BigUint) -> BigUint{
    let number: BigUint = BigUint::from(2).pow(digits-1);
    loop {
        number += 1;
        if tests::is_prime(number) {return number;}
    }
}
