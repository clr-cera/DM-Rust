mod test_lib;
use test_lib::{jobs, JobChoice, receive_work};

fn main() {
    loop {
        match receive_work() {
            JobChoice::Crypto => jobs::cryptography_job(),
            JobChoice::CheckPrime => jobs::check_prime_job(),
            JobChoice::CheckPseudoPrime => jobs::check_pseudoprime_job(),
            JobChoice::CheckStrongPseudoPrime => jobs::check_strong_pseudoprime_job(),
            JobChoice::GeneratePrimeFrom => jobs::generate_prime_job(),
            JobChoice::Quit => break,
        }
    }
}
