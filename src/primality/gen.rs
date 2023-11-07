use super::tests::is_prime;

pub fn generate_first_prime_from(digits: u16) -> u128{
    let start: u128 = 2u128.pow((digits-1) as u32);
    for number in start as u128..=start + ((start).ilog(3) * 3) as u128 {
        if is_prime(number as u128) {return number;}
    }
    return 0;
}
