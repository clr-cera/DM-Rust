mod test_lib;
use test_lib::jobs;

fn main() {
    loop {
        match test_lib::receive_work() {
            0 => jobs::cryptography_job(),
            1 => jobs::check_prime_job(),
            2 => jobs::check_pseudoprime_job(),
            3 => jobs::generate_prime_job(),
            4 => break,
            _ => (),
        }
    }
}

