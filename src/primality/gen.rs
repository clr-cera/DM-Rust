use super::tests;

use num_bigint::BigUint;

/// This function generates the first prime with informed digits. It uses miller-rabin test of
/// composites.
pub fn generate_first_prime_from(digits: &BigUint) -> BigUint{
    let mut number: BigUint = BigUint::from(2u32).pow((digits-1u32).to_u32_digits()[0]);
    loop {
        number += 1u32;
        if tests::is_prime(&number) {return number;}
    }
}
