use super::tests;

/// This function generates the first prime with informed digits. It uses miller-rabin test of
/// composites.
pub fn generate_first_prime_from(digits: u16) -> u128{
    let start: u128 = 2u128.pow((digits-1) as u32);
    for number in start as u128..=start + ((start).ilog(3) * 6) as u128 {
        if tests::is_prime(number as u128) {return number;}
    }
    return 0;
}
